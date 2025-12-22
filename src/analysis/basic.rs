//! Basic FFT-based analyzer for spek-core.
//!
//! Implements the full signal pipeline:
//! PCM → windowing → FFT → power → dBFS → scaling → spectrogram

use crate::analysis::{
    AnalysisError, AnalysisSettings, Analyzer, Spectrogram, SpectrogramSet, WindowFunction,
};
use crate::audio::AudioBuffer;

use rustfft::{num_complex::Complex, FftPlanner};

/// Reference FFT-based analyzer.
pub struct BasicAnalyzer;

impl Analyzer for BasicAnalyzer {
    fn analyze(
        &self,
        audio: &AudioBuffer<'_>,
        settings: &AnalysisSettings,
    ) -> Result<SpectrogramSet, AnalysisError> {
        if settings.fft_size == 0 || settings.hop_size == 0 {
            return Err(AnalysisError::InvalidParameters);
        }

        let channels = audio.meta.channels as usize;
        let samples = audio.samples;
        let fft_size = settings.fft_size;
        let hop = settings.hop_size;

        let frames = (samples.len() / channels).saturating_sub(fft_size) / hop;
        if frames == 0 {
            return Err(AnalysisError::ProcessingFailed);
        }

        let freq_bins = fft_size / 2;
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(fft_size);

        let window = build_window(settings.window, fft_size);

        let mut result = Vec::with_capacity(channels);

        for ch in 0..channels {
            let mut spec = vec![vec![0.0f32; frames]; freq_bins];

            let mut buffer = vec![Complex::ZERO; fft_size];

            for t in 0..frames {
                let base = t * hop * channels + ch;

                for i in 0..fft_size {
                    let idx = base + i * channels;
                    let sample = samples.get(idx).copied().unwrap_or(0.0);
                    buffer[i].re = sample * window[i];
                    buffer[i].im = 0.0;
                }

                fft.process(&mut buffer);

                for f in 0..freq_bins {
                    let power = buffer[f].norm_sqr();
                    let db = power_to_db(power, settings.min_db);
                    spec[f][t] = scale(db, settings.min_db, &settings.scale);
                }
            }

            result.push(Spectrogram {
                freq_bins,
                time_bins: frames,
                data: spec,
            });
        }

        Ok(SpectrogramSet { channels: result })
    }
}

/// Convert power value to dBFS.
fn power_to_db(power: f32, min_db: f32) -> f32 {
    let epsilon = 1e-12;
    let db = 10.0 * (power + epsilon).log10();
    db.max(min_db)
}

/// Apply intensity scaling and normalize to 0.0–1.0.
fn scale(db: f32, min_db: f32, scale: &crate::analysis::IntensityScale) -> f32 {
    let norm = (db - min_db) / -min_db;
    let v = match scale {
        crate::analysis::IntensityScale::Linear => norm,
        crate::analysis::IntensityScale::Sqrt => norm.sqrt(),
        crate::analysis::IntensityScale::Cbrt => norm.cbrt(),
        crate::analysis::IntensityScale::Log => (norm * 9.0 + 1.0).log10(),
        crate::analysis::IntensityScale::Power(p) => norm.powf(*p),
    };
    v.clamp(0.0, 1.0)
}

/// Build window coefficients.
fn build_window(window: WindowFunction, size: usize) -> Vec<f32> {
    match window {
        WindowFunction::Rectangular => vec![1.0; size],
        WindowFunction::Hann => (0..size)
            .map(|i| {
                0.5 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / size as f32).cos()
            })
            .collect(),
        WindowFunction::Hamming => (0..size)
            .map(|i| {
                0.54 - 0.46 * (2.0 * std::f32::consts::PI * i as f32 / size as f32).cos()
            })
            .collect(),
        WindowFunction::Blackman => (0..size)
            .map(|i| {
                0.42
                    - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / size as f32).cos()
                    + 0.08 * (4.0 * std::f32::consts::PI * i as f32 / size as f32).cos()
            })
            .collect(),
        _ => vec![1.0; size], // safe fallback
    }
}
