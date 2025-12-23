//! FFT-based signal analysis for spek-core.
//!
//! This module implements the numerical spectrogram pipeline:
//! PCM -> windowing -> FFT -> power -> dBFS -> scaling -> normalization
//!
//! This module has NO rendering logic and NO platform dependencies.

use crate::analysis::{
    AnalysisError, AnalysisSettings, Analyzer, IntensityScale, Spectrogram, SpectrogramSet,
    WindowFunction,
};
use crate::audio::AudioBuffer;

use std::f32::consts::PI;

/// Basic FFT analyzer.
///
/// This is a single-shot, deterministic analyzer.
/// No internal state is kept between calls.
pub struct FftAnalyzer;

impl FftAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

impl Analyzer for FftAnalyzer {
    fn analyze(
        &self,
        audio: &AudioBuffer,
        settings: &AnalysisSettings,
    ) -> Result<SpectrogramSet, AnalysisError> {
        let channels = audio.meta.channels as usize;
        let fft_size = settings.fft_size;
        let hop = settings.hop_size;

        if fft_size == 0 || hop == 0 || audio.samples.is_empty() {
            return Err(AnalysisError::InvalidParameters);
        }

        let samples_per_channel = audio.samples.len() / channels;
        let time_bins = (samples_per_channel.saturating_sub(fft_size)) / hop;
        let freq_bins = fft_size / 2;

        let window = build_window(settings.window, fft_size);

        let mut result = Vec::with_capacity(channels);

        for ch in 0..channels {
            let mut spec = Spectrogram {
                freq_bins,
                time_bins,
                data: vec![vec![0.0; time_bins]; freq_bins],
            };

            for t in 0..time_bins {
                let offset = t * hop;
                let mut re = vec![0.0f32; fft_size];
                let mut im = vec![0.0f32; fft_size];

                for i in 0..fft_size {
                    let idx = (offset + i) * channels + ch;
                    let sample = audio.samples.get(idx).copied().unwrap_or(0.0);
                    re[i] = sample * window[i];
                }

                fft_inplace(&mut re, &mut im);

                for f in 0..freq_bins {
                    let power = re[f] * re[f] + im[f] * im[f];
                    let db = power_to_db(power, settings.min_db);
                    let norm = normalize_db(db, settings.min_db);
                    let scaled = apply_scale(norm, settings.scale);
                    spec.data[f][t] = scaled;
                }
            }

            result.push(spec);
        }

        Ok(SpectrogramSet { channels: result })
    }
}

/// Build a window function.
fn build_window(kind: WindowFunction, size: usize) -> Vec<f32> {
    match kind {
        WindowFunction::Rectangular => vec![1.0; size],
        WindowFunction::Hann => (0..size)
            .map(|i| 0.5 - 0.5 * (2.0 * PI * i as f32 / size as f32).cos())
            .collect(),
        WindowFunction::Hamming => (0..size)
            .map(|i| 0.54 - 0.46 * (2.0 * PI * i as f32 / size as f32).cos())
            .collect(),
        WindowFunction::Blackman => (0..size)
            .map(|i| {
                0.42
                    - 0.5 * (2.0 * PI * i as f32 / size as f32).cos()
                    + 0.08 * (4.0 * PI * i as f32 / size as f32).cos()
            })
            .collect(),
        _ => vec![1.0; size], // fallback
    }
}

/// In-place radix-2 FFT (Cooley–Tukey).
///
/// NOTE:
/// This is a minimal reference implementation.
/// It will be replaced by a faster FFT backend later.
fn fft_inplace(re: &mut [f32], im: &mut [f32]) {
    let n = re.len();
    let mut j = 0;

    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;

        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }

    let mut len = 2;
    while len <= n {
        let half = len / 2;
        let step = PI * 2.0 / len as f32;
        for i in (0..n).step_by(len) {
            for j in 0..half {
                let angle = step * j as f32;
                let wr = angle.cos();
                let wi = -angle.sin();

                let k = i + j;
                let l = k + half;

                let tr = wr * re[l] - wi * im[l];
                let ti = wr * im[l] + wi * re[l];

                re[l] = re[k] - tr;
                im[l] = im[k] - ti;
                re[k] += tr;
                im[k] += ti;
            }
        }
        len <<= 1;
    }
}

/// Convert power to dBFS.
fn power_to_db(power: f32, min_db: f32) -> f32 {
    let eps = 1e-12;
    let db = 10.0 * (power + eps).log10();
    db.max(min_db)
}

/// Normalize dBFS into 0.0–1.0.
///
/// Spek-style normalization:
/// - min_db -> 0.0
/// - 0 dBFS -> 1.0
fn normalize_db(db: f32, min_db: f32) -> f32 {
    ((db - min_db) / (0.0 - min_db)).clamp(0.0, 1.0)
}

/// Apply intensity scaling.
fn apply_scale(v: f32, scale: IntensityScale) -> f32 {
    match scale {
        IntensityScale::Linear => v,
        IntensityScale::Sqrt => v.sqrt(),
        IntensityScale::Cbrt => v.cbrt(),

        // Spek-like perceptual log scaling
        IntensityScale::Log => {
            // Boost low-level energy so it becomes visible
            (v * 1000.0 + 1.0).log10() / 3.0
        }

        IntensityScale::Power(p) => v.powf(p),
    }
}
