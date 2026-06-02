use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::Path;

use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

const OPUS_PRESKIP_SAMPLES: usize = 3840;
const STDIN_MAX_BYTES: u64 = 2 * 1024 * 1024 * 1024;

pub struct PcmData {
    pub samples: Vec<i16>,
    pub sample_rate: u32,
    pub channels: usize,
}

impl PcmData {
    pub fn duration_seconds(&self) -> f64 {
        if self.sample_rate == 0 || self.channels == 0 {
            return 0.0;
        }
        self.samples.len() as f64 / (self.sample_rate as f64 * self.channels as f64)
    }
}

pub fn decode_file(path: &Path) -> Result<PcmData, crate::error::Error> {
    let file = std::fs::File::open(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            crate::error::Error::InputNotFound {
                path: path.display().to_string(),
            }
        } else {
            crate::error::Error::Io(e)
        }
    })?;

    let mut header = [0u8; 12];
    let header_len = match (&file).read(&mut header) {
        Ok(n) => n,
        Err(e) => return Err(crate::error::Error::Io(e)),
    };
    if let Err(e) = (&file).seek(SeekFrom::Start(0)) {
        return Err(crate::error::Error::Io(e));
    }

    if header_len >= 4 && is_ogg_opus_magic(&header[..header_len]) {
        return decode_ogg_opus(file);
    }

    let source = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    match decode_stream(source, hint) {
        Ok(pcm) => Ok(pcm),
        Err(crate::error::Error::AudioDecode(ref e))
            if e.to_string().contains("unsupported codec") =>
        {
            tracing::info!("symphonia unsupported codec, trying OGG/Opus fallback");
            let file2 = std::fs::File::open(path).map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    crate::error::Error::InputNotFound {
                        path: path.display().to_string(),
                    }
                } else {
                    crate::error::Error::Io(e)
                }
            })?;
            decode_ogg_opus(file2)
        }
        Err(e) => Err(e),
    }
}

pub fn decode_stdin(format_hint: Option<&str>) -> Result<PcmData, crate::error::Error> {
    let mut buf = Vec::new();
    let mut handle = std::io::stdin().take(STDIN_MAX_BYTES + 1);
    handle
        .read_to_end(&mut buf)
        .map_err(crate::error::Error::Io)?;

    if buf.is_empty() {
        return Err(crate::error::Error::NoInput);
    }
    if buf.len() as u64 > STDIN_MAX_BYTES {
        return Err(crate::error::Error::Config(format!(
            "stdin input exceeds maximum size of {STDIN_MAX_BYTES} bytes"
        )));
    }

    if is_ogg_opus_magic(&buf[..buf.len().min(12)]) {
        return decode_ogg_opus(Cursor::new(buf));
    }

    let source = MediaSourceStream::new(Box::new(Cursor::new(buf.clone())), Default::default());

    let mut hint = Hint::new();
    if let Some(fmt) = format_hint {
        hint.with_extension(fmt);
    }

    match decode_stream(source, hint) {
        Ok(pcm) => Ok(pcm),
        Err(crate::error::Error::AudioDecode(ref e))
            if e.to_string().contains("unsupported codec") =>
        {
            tracing::info!("symphonia unsupported codec, trying OGG/Opus fallback");
            decode_ogg_opus(Cursor::new(buf))
        }
        Err(e) => Err(e),
    }
}

pub fn is_ogg_opus_magic(header: &[u8]) -> bool {
    if header.len() < 4 {
        return false;
    }
    &header[..4] == b"OggS"
}

fn decode_stream(source: MediaSourceStream, hint: Hint) -> Result<PcmData, crate::error::Error> {
    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            source,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| crate::error::Error::AudioDecode(anyhow::anyhow!("probe failed: {e}")))?;

    let mut reader = probed.format;

    let track = reader
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| crate::error::Error::AudioDecode(anyhow::anyhow!("no audio track found")))?;

    let track_id = track.id;
    let codec_params = track.codec_params.clone();

    let sample_rate = codec_params
        .sample_rate
        .ok_or_else(|| crate::error::Error::AudioDecode(anyhow::anyhow!("unknown sample rate")))?;

    let channels = codec_params.channels.map(|c| c.count()).unwrap_or(2);

    let mut decoder = symphonia::default::get_codecs()
        .make(&codec_params, &DecoderOptions::default())
        .map_err(|e| crate::error::Error::AudioDecode(anyhow::anyhow!("codec init failed: {e}")))?;

    let mut all_samples: Vec<i16> = Vec::new();

    loop {
        let packet = match reader.next_packet() {
            Ok(p) => p,
            Err(symphonia::core::errors::Error::IoError(e))
                if e.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(_) => continue,
        };

        if packet.track_id() != track_id {
            continue;
        }

        let audio_buf = match decoder.decode(&packet) {
            Ok(buf) => buf,
            Err(_) => continue,
        };

        extract_i16_samples(&audio_buf, &mut all_samples);
    }

    if all_samples.is_empty() {
        return Err(crate::error::Error::AudioDecode(anyhow::anyhow!(
            "no audio samples decoded"
        )));
    }

    Ok(PcmData {
        samples: all_samples,
        sample_rate,
        channels,
    })
}

pub fn to_mono(samples: &[i16], channels: usize) -> Vec<i16> {
    if channels == 1 {
        return samples.to_vec();
    }

    let num_frames = samples.len() / channels;
    let mut mono = Vec::with_capacity(num_frames);

    for frame in 0..num_frames {
        let mut sum: i32 = 0;
        for ch in 0..channels {
            sum += samples[frame * channels + ch] as i32;
        }
        let avg = sum / channels as i32;
        mono.push(avg.clamp(i16::MIN as i32, i16::MAX as i32) as i16);
    }

    mono
}

pub fn i16_to_f32(samples: &[i16]) -> Vec<f32> {
    samples.iter().map(|&s| s as f32 / 32768.0).collect()
}

fn decode_ogg_opus<R: Read + Seek>(mut reader: R) -> Result<PcmData, crate::error::Error> {
    use ogg::reading::PacketReader;

    let mut ogg_reader = PacketReader::new(&mut reader);
    let mut channels = 1u8;
    let mut pre_skip = OPUS_PRESKIP_SAMPLES;
    let mut header_packets = 0u8;

    while header_packets < 2 {
        let pkt = ogg_reader
            .read_packet_expected()
            .map_err(|e| crate::error::Error::AudioDecode(anyhow::anyhow!("ogg header: {e}")))?;

        if header_packets == 0 && pkt.data.len() >= 16 && &pkt.data[..8] == b"OpusHead" {
            channels = pkt.data[9];
            pre_skip = u32::from_le_bytes([pkt.data[10], pkt.data[11], pkt.data[12], pkt.data[13]])
                as usize;
        }
        header_packets += 1;
    }

    let channels_usize = channels.max(1) as usize;
    let output_rate = 48000;

    let mut decoder = opus_decoder::OpusDecoder::new(output_rate, channels_usize)
        .map_err(|e| crate::error::Error::AudioDecode(anyhow::anyhow!("opus init: {e:?}")))?;

    let max_frame = opus_decoder::OpusDecoder::MAX_FRAME_SIZE_48K;
    let mut pcm_buf = vec![0i16; max_frame * channels_usize];
    let mut all_samples: Vec<i16> = Vec::new();
    let mut samples_to_skip = pre_skip;

    loop {
        let pkt = match ogg_reader.read_packet() {
            Ok(Some(p)) => p,
            Ok(None) => break,
            Err(_) => continue,
        };

        match decoder.decode(&pkt.data, &mut pcm_buf, false) {
            Ok(samples_per_channel) => {
                let total = samples_per_channel * channels_usize;
                let slice = &pcm_buf[..total];

                if samples_to_skip >= total {
                    samples_to_skip -= total;
                } else if samples_to_skip > 0 {
                    let kept = &slice[samples_to_skip..];
                    all_samples.extend_from_slice(kept);
                    samples_to_skip = 0;
                } else {
                    all_samples.extend_from_slice(slice);
                }
            }
            Err(_) => continue,
        }
    }

    if all_samples.is_empty() {
        return Err(crate::error::Error::AudioDecode(anyhow::anyhow!(
            "no audio samples decoded from OGG/Opus"
        )));
    }

    tracing::info!(
        samples = all_samples.len(),
        channels = channels_usize,
        preskip_discarded = pre_skip,
        "OGG/Opus decoded via fallback"
    );

    Ok(PcmData {
        samples: all_samples,
        sample_rate: output_rate,
        channels: channels_usize,
    })
}

fn extract_i16_samples(buffer: &AudioBufferRef, dest: &mut Vec<i16>) {
    match buffer {
        AudioBufferRef::U8(buf) => {
            let ch = buf.spec().channels.count();
            let frames = buf.frames();
            dest.reserve(frames * ch);
            for f in 0..frames {
                for c in 0..ch {
                    dest.push(((buf.chan(c)[f] as i32 - 128) * 256) as i16);
                }
            }
        }
        AudioBufferRef::S16(buf) => {
            let ch = buf.spec().channels.count();
            let frames = buf.frames();
            dest.reserve(frames * ch);
            for f in 0..frames {
                for c in 0..ch {
                    dest.push(buf.chan(c)[f]);
                }
            }
        }
        AudioBufferRef::S32(buf) => {
            let ch = buf.spec().channels.count();
            let frames = buf.frames();
            dest.reserve(frames * ch);
            for f in 0..frames {
                for c in 0..ch {
                    dest.push((buf.chan(c)[f] >> 16) as i16);
                }
            }
        }
        AudioBufferRef::F32(buf) => {
            let ch = buf.spec().channels.count();
            let frames = buf.frames();
            dest.reserve(frames * ch);
            for f in 0..frames {
                for c in 0..ch {
                    let v = buf.chan(c)[f].clamp(-1.0, 1.0);
                    dest.push((v * 32767.0) as i16);
                }
            }
        }
        AudioBufferRef::F64(buf) => {
            let ch = buf.spec().channels.count();
            let frames = buf.frames();
            dest.reserve(frames * ch);
            for f in 0..frames {
                for c in 0..ch {
                    let v = buf.chan(c)[f].clamp(-1.0, 1.0);
                    dest.push((v * 32767.0) as i16);
                }
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_mono_passthrough_single_channel() {
        let samples = vec![100i16, 200, 300];
        let result = to_mono(&samples, 1);
        assert_eq!(result, samples);
    }

    #[test]
    fn to_mono_averages_stereo() {
        let samples = vec![100i16, 200, 300, 400];
        let result = to_mono(&samples, 2);
        assert_eq!(result, vec![150, 350]);
    }

    #[test]
    fn i16_to_f32_converts_correctly() {
        let samples = vec![0i16, 32767, -32768];
        let result = i16_to_f32(&samples);
        assert!((result[0] - 0.0).abs() < 0.001);
        assert!((result[1] - 1.0).abs() < 0.001);
        assert!((result[2] - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn opus_magic_detected() {
        let ogg = b"OggS\x00\x02\x00\x00\x00\x00\x00\x00";
        assert!(is_ogg_opus_magic(ogg));
    }

    #[test]
    fn non_opus_not_detected() {
        let wav = b"RIFF\x00\x00\x00\x00";
        assert!(!is_ogg_opus_magic(wav));
    }

    #[test]
    fn short_buffer_not_detected() {
        let short = b"Og";
        assert!(!is_ogg_opus_magic(short));
    }

    #[test]
    fn pcm_data_duration_computed_correctly() {
        let pcm = PcmData {
            samples: vec![0i16; 16000 * 2],
            sample_rate: 16000,
            channels: 1,
        };
        assert!((pcm.duration_seconds() - 2.0).abs() < 0.001);
    }
}
