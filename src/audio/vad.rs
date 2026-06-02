use earshot::Detector;
use regex::Regex;
use std::sync::OnceLock;

const FRAME_SIZE: usize = 256;
const MAX_CHUNK_SAMPLES: usize = 25 * 16000; // 25s
const MIN_SILENCE_SAMPLES: usize = 19200; // 1.2s
const MIN_CHUNK_SAMPLES: usize = 24000; // 1.5s
const PADDING_SAMPLES: usize = 3200; // 0.2s

struct SpeechSegment {
    start: usize,
    end: usize,
}

pub fn detect_speech_segments(samples: &[i16], threshold: f32) -> Vec<(usize, usize)> {
    let raw_segments = detect_raw_segments(samples, threshold);

    tracing::info!(
        segments = raw_segments.len(),
        "VAD speech segments detected"
    );

    if raw_segments.is_empty() {
        return Vec::new();
    }

    group_segments(&raw_segments, samples.len())
}

fn detect_raw_segments(samples: &[i16], threshold: f32) -> Vec<SpeechSegment> {
    let mut detector = Detector::default();
    let mut segments: Vec<SpeechSegment> = Vec::new();
    let mut in_speech = false;
    let mut speech_start: usize = 0;

    let total_frames = samples.len() / FRAME_SIZE;

    for frame_idx in 0..total_frames {
        let offset = frame_idx * FRAME_SIZE;
        let frame = &samples[offset..offset + FRAME_SIZE];

        let score = detector.predict_i16(frame);
        let has_voice = score >= threshold;

        match (in_speech, has_voice) {
            (false, true) => {
                speech_start = offset;
                in_speech = true;
            }
            (true, false) => {
                segments.push(SpeechSegment {
                    start: speech_start,
                    end: offset,
                });
                in_speech = false;
            }
            _ => {}
        }
    }

    if in_speech {
        segments.push(SpeechSegment {
            start: speech_start,
            end: (total_frames * FRAME_SIZE).min(samples.len()),
        });
    }

    segments
}

fn group_segments(segments: &[SpeechSegment], total_samples: usize) -> Vec<(usize, usize)> {
    let mut chunks: Vec<(usize, usize)> = Vec::new();
    let mut chunk_start = segments[0].start;
    let mut chunk_end = segments[0].end;

    for seg in segments.iter().skip(1) {
        let silence = seg.start.saturating_sub(chunk_end);
        let projected_size = seg.end.saturating_sub(chunk_start);

        if silence < MIN_SILENCE_SAMPLES && projected_size <= MAX_CHUNK_SAMPLES {
            chunk_end = seg.end;
        } else {
            chunks.push((chunk_start, chunk_end));
            chunk_start = seg.start;
            chunk_end = seg.end;
        }
    }

    chunks.push((chunk_start, chunk_end));

    // Merge short chunks into neighbors
    let mut consolidated: Vec<(usize, usize)> = Vec::with_capacity(chunks.len());

    for (start, end) in chunks {
        let duration = end.saturating_sub(start);

        if duration < MIN_CHUNK_SAMPLES {
            if let Some(last) = consolidated.last_mut() {
                last.1 = end;
            } else {
                consolidated.push((start, end));
            }
        } else if let Some(&last) = consolidated.last() {
            let last_dur = last.1.saturating_sub(last.0);

            if last_dur < MIN_CHUNK_SAMPLES {
                if let Some((prev_start, _)) = consolidated.pop() {
                    consolidated.push((prev_start, end));
                }
            } else {
                consolidated.push((start, end));
            }
        } else {
            consolidated.push((start, end));
        }
    }

    tracing::info!(chunks = consolidated.len(), "VAD chunks grouped");

    // Apply padding
    consolidated
        .into_iter()
        .map(|(s, e)| {
            let s_pad = s.saturating_sub(PADDING_SAMPLES);
            let e_pad = (e + PADDING_SAMPLES).min(total_samples);
            (s_pad, e_pad)
        })
        .collect()
}

static HALLUCINATION_REGEX: OnceLock<Regex> = OnceLock::new();

fn hallucination_regex() -> &'static Regex {
    HALLUCINATION_REGEX.get_or_init(|| {
        Regex::new(
            r"(?im)^[ \t]*(legenda(?:s)? por\b|amara\.org|legendas? pela comunidade|www\.|inscreva-se|subscribe|transcri(?:ção|cao) por\b|transcri(?:ção|cao) e legendas?\b).*$"
        )
        .expect("hallucination regex is a static literal")
    })
}

pub fn filter_hallucinations(text: &str) -> String {
    let re = hallucination_regex();
    let filtered = re.replace_all(text, "");

    let mut result = String::with_capacity(filtered.len());
    let mut prev_empty = false;

    for line in filtered.lines() {
        let empty = line.trim().is_empty();
        if empty {
            if !prev_empty {
                result.push('\n');
            }
            prev_empty = true;
        } else {
            result.push_str(line);
            result.push('\n');
            prev_empty = false;
        }
    }

    result.trim_end().to_string()
}

pub fn collapse_consecutive_repeats(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut last_nonempty: Option<String> = None;

    for line in text.lines() {
        let content = line.trim();

        if content.is_empty() {
            result.push('\n');
            last_nonempty = None;
        } else if last_nonempty.as_deref() == Some(content) {
            // Skip consecutive duplicate
        } else {
            result.push_str(line);
            result.push('\n');
            last_nonempty = Some(content.to_string());
        }
    }

    result.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_hallucinations_removes_known_patterns() {
        let input = "Hello world\nlegendas por comunidade\nGoodbye";
        let result = filter_hallucinations(input);
        assert!(result.contains("Hello world"));
        assert!(result.contains("Goodbye"));
        assert!(!result.contains("legendas por"));
    }

    #[test]
    fn filter_hallucinations_preserves_clean_text() {
        let input = "This is clean text\nWith multiple lines";
        let result = filter_hallucinations(input);
        assert_eq!(result, input);
    }

    #[test]
    fn collapse_consecutive_repeats_removes_duplicates() {
        let input = "Hello\nHello\nWorld\nWorld\nWorld";
        let result = collapse_consecutive_repeats(input);
        assert_eq!(result, "Hello\nWorld");
    }

    #[test]
    fn collapse_consecutive_repeats_keeps_non_consecutive() {
        let input = "Hello\nWorld\nHello";
        let result = collapse_consecutive_repeats(input);
        assert_eq!(result, input);
    }

    #[test]
    fn filter_hallucinations_handles_empty_input() {
        assert_eq!(filter_hallucinations(""), "");
    }

    #[test]
    fn filter_hallucinations_handles_only_whitespace() {
        let input = "   \n\n\t\n   ";
        let result = filter_hallucinations(input);
        assert!(result.trim().is_empty() || result.is_empty());
    }

    #[test]
    fn filter_hallucinations_case_insensitive_legendas() {
        let input = "Real text\nLEGENDA POR comunidade\nMore real text";
        let result = filter_hallucinations(input);
        assert!(!result.contains("legenda"));
        assert!(!result.contains("LEGENDA"));
        assert!(result.contains("Real text"));
        assert!(result.contains("More real text"));
    }

    #[test]
    fn filter_hallucinations_strips_amara_url() {
        let input = "Hello\namara.org\nWorld";
        let result = filter_hallucinations(input);
        assert!(!result.contains("amara"));
    }

    #[test]
    fn filter_hallucinations_strips_subscribe() {
        let input = "Real text\ninscreva-se no canal\nMore text";
        let result = filter_hallucinations(input);
        assert!(!result.contains("inscreva"));
    }

    #[test]
    fn filter_hallucinations_strips_www_url() {
        let input = "Hello\nwww.example.com\nWorld";
        let result = filter_hallucinations(input);
        assert!(!result.contains("www."));
    }

    #[test]
    fn filter_hallucinations_strips_transcricao_por() {
        let input = "Real text\ntranscrição por fulano\nMore text";
        let result = filter_hallucinations(input);
        assert!(!result.contains("transcrição por"));
    }

    #[test]
    fn filter_hallucinations_strips_transcricao_e_legendas() {
        let input = "Real text\ntranscrição e legendas\nMore text";
        let result = filter_hallucinations(input);
        assert!(!result.contains("transcrição e legendas"));
    }

    #[test]
    fn collapse_consecutive_repeats_handles_empty_input() {
        assert_eq!(collapse_consecutive_repeats(""), "");
    }

    #[test]
    fn collapse_consecutive_repeats_collapses_three_in_a_row() {
        let input = "Hello\nHello\nHello\nWorld";
        let result = collapse_consecutive_repeats(input);
        assert_eq!(result, "Hello\nWorld");
    }

    #[test]
    fn collapse_consecutive_repeats_preserves_blank_lines() {
        let input = "Hello\n\nWorld";
        let result = collapse_consecutive_repeats(input);
        assert!(result.contains("Hello"));
        assert!(result.contains("World"));
    }
}
