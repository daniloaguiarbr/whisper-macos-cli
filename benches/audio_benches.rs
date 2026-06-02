use criterion::{Criterion, black_box, criterion_group, criterion_main};

use whisper_macos_cli::audio::vad::{
    collapse_consecutive_repeats, detect_speech_segments, filter_hallucinations,
};
use whisper_macos_cli::audio::{decode, resample};
use whisper_macos_cli::language::detect::map_to_whisper_code;

fn bench_vad_silent(c: &mut Criterion) {
    let samples: Vec<i16> = vec![0; 16000];
    c.bench_function("vad/silent_1s", |b| {
        b.iter(|| detect_speech_segments(black_box(&samples), 0.5))
    });
}

fn bench_vad_loud(c: &mut Criterion) {
    let samples: Vec<i16> = (0..16000)
        .map(|i| if (i / 256) % 2 == 0 { 10000 } else { -10000 })
        .collect();
    c.bench_function("vad/loud_1s", |b| {
        b.iter(|| detect_speech_segments(black_box(&samples), 0.5))
    });
}

fn bench_filter_hallucinations(c: &mut Criterion) {
    let text = "Real text\nlegendas por comunidade\nMore real text\namara.org\nEven more text";
    c.bench_function("filter/hallucinations", |b| {
        b.iter(|| filter_hallucinations(black_box(text)))
    });
}

fn bench_collapse_repeats(c: &mut Criterion) {
    let text = "Hello\nHello\nWorld\nWorld\nWorld\nGoodbye\nGoodbye";
    c.bench_function("filter/collapse_repeats", |b| {
        b.iter(|| collapse_consecutive_repeats(black_box(text)))
    });
}

fn bench_to_mono_stereo(c: &mut Criterion) {
    let samples: Vec<i16> = (0..32000).map(|i| (i % 1000) as i16).collect();
    c.bench_function("audio/to_mono_stereo_1s", |b| {
        b.iter(|| decode::to_mono(black_box(&samples), 2))
    });
}

fn bench_to_mono_six_channel(c: &mut Criterion) {
    let samples: Vec<i16> = (0..96000).map(|i| (i % 1000) as i16).collect();
    c.bench_function("audio/to_mono_six_channel_1s", |b| {
        b.iter(|| decode::to_mono(black_box(&samples), 6))
    });
}

fn bench_i16_to_f32(c: &mut Criterion) {
    let samples: Vec<i16> = (0..16000).map(|i| (i % 1000) as i16).collect();
    c.bench_function("audio/i16_to_f32_1s", |b| {
        b.iter(|| decode::i16_to_f32(black_box(&samples)))
    });
}

fn bench_resample_passthrough(c: &mut Criterion) {
    let samples: Vec<i16> = (0..16000).map(|i| (i % 1000) as i16).collect();
    c.bench_function("audio/resample_passthrough_16khz", |b| {
        b.iter(|| resample::resample_to_16khz(black_box(&samples), 16000))
    });
}

fn bench_resample_44100_to_16000(c: &mut Criterion) {
    let samples: Vec<i16> = (0..44100).map(|i| (i % 1000) as i16).collect();
    c.bench_function("audio/resample_44100_to_16000_1s", |b| {
        b.iter(|| resample::resample_to_16khz(black_box(&samples), 44100))
    });
}

fn bench_map_to_whisper_code(c: &mut Criterion) {
    let codes = ["pt", "en", "es", "fr", "de", "it", "ja", "zh", "ru", "ar"];
    c.bench_function("language/map_to_whisper_code", |b| {
        b.iter(|| {
            for code in &codes {
                let _ = map_to_whisper_code(black_box(code));
            }
        })
    });
}

criterion_group!(
    benches,
    bench_vad_silent,
    bench_vad_loud,
    bench_filter_hallucinations,
    bench_collapse_repeats,
    bench_to_mono_stereo,
    bench_to_mono_six_channel,
    bench_i16_to_f32,
    bench_resample_passthrough,
    bench_resample_44100_to_16000,
    bench_map_to_whisper_code
);
criterion_main!(benches);
