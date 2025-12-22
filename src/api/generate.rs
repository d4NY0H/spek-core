//! Public spectrogram generation API.
//!
//! This module defines the single, stable entry point into spek-core.
//! It wires together the full pipeline:
//!
//! audio → analysis → render → legend → result
//!
//! No UI logic, no platform-specific code.

use crate::api::result::{SpectrogramResult, ImageBuffer as ApiImageBuffer};
use crate::api::settings::SpekSettings;

use crate::analysis::{Analyzer, AnalysisSettings, IntensityScale, WindowFunction};
use crate::audio::AudioSource;
use crate::render::{RenderSettings as CoreRenderSettings, Renderer};
use crate::legend::LegendRenderer;
use crate::color::ColorMapper;

/// Generate a spectrogram image including legend.
///
/// This is the ONLY public entry point of spek-core.
///
/// One call → one deterministic result.
pub fn generate_spectrogram(
    source: &dyn AudioSource,
    analyzer: &dyn Analyzer,
    renderer: &dyn Renderer,
    color_mapper: &dyn ColorMapper,
    legend: &dyn LegendRenderer,
    settings: &SpekSettings,
) -> Result<SpectrogramResult, GenerateError> {
    // ---------------------------------------------------------------------
    // 1. Load audio
    // ---------------------------------------------------------------------
    let audio = source.load().map_err(|_| GenerateError::DecodeFailed)?;

    let duration_seconds =
        audio.meta.total_samples as f64 / audio.meta.sample_rate as f64;

    // ---------------------------------------------------------------------
    // 2. Build analysis settings
    // ---------------------------------------------------------------------
    let analysis_settings = AnalysisSettings {
        fft_size: settings.spectrogram.fft_size,
        hop_size: settings.spectrogram.hop_size,
        window: map_window(settings.spectrogram.window),
        scale: map_scale(settings.spectrogram.scale),
        min_db: settings.spectrogram.min_db,
    };

    // ---------------------------------------------------------------------
    // 3. Run signal analysis
    // ---------------------------------------------------------------------
    let spectrograms = analyzer
        .analyze(&audio, &analysis_settings)
        .map_err(|_| GenerateError::AnalysisFailed)?;

    // ---------------------------------------------------------------------
    // 4. Render spectrogram image (without legend)
    // ---------------------------------------------------------------------
    let render_settings = CoreRenderSettings {
        width: settings.render.width,
        height: settings.render.height,
        orientation: crate::render::Orientation::Vertical,
        channels: crate::render::ChannelLayout::Combined,
    };

    let mut image = renderer
        .render(&spectrograms, &render_settings, color_mapper)
        .map_err(|_| GenerateError::RenderFailed)?;

    // ---------------------------------------------------------------------
    // 5. Render legend overlay
    // ---------------------------------------------------------------------
    let legend_commands = legend.generate(
        &crate::legend::LegendSettings {
            font_size: 14,
            freq_ticks: 10,
            time_ticks: 10,
            db_ticks: 6,
        },
        &crate::legend::LegendContext {
            audio: audio.meta.clone(),
            duration_sec: duration_seconds,
            min_db: settings.spectrogram.min_db,
            max_db: settings.spectrogram.max_db,
        },
        crate::legend::LegendMargins {
            left: 80,
            right: 100,
            top: 60,
            bottom: 60,
        },
        image.width as u32,
        image.height as u32,
    );

    crate::legend::overlay::apply_legend_overlay(&mut image, &legend_commands);

    // ---------------------------------------------------------------------
    // 6. Assemble result (API-level ImageBuffer)
    // ---------------------------------------------------------------------
    Ok(SpectrogramResult {
        image: ApiImageBuffer {
            width: image.width as u32,
            height: image.height as u32,
            data: image.data,
        },
        duration_seconds,
        sample_rate: audio.meta.sample_rate,
        channels: audio.meta.channels as u32,
    })
}

/// Public error type for spectrogram generation.
#[derive(Debug)]
pub enum GenerateError {
    DecodeFailed,
    AnalysisFailed,
    RenderFailed,
}

/// Map public window enum to analysis window enum.
fn map_window(w: crate::api::settings::WindowFunction) -> WindowFunction {
    match w {
        crate::api::settings::WindowFunction::Rectangular => WindowFunction::Rectangular,
        crate::api::settings::WindowFunction::Hann => WindowFunction::Hann,
        crate::api::settings::WindowFunction::Hamming => WindowFunction::Hamming,
        crate::api::settings::WindowFunction::Blackman => WindowFunction::Blackman,
    }
}

/// Map public scale enum to analysis scale enum.
fn map_scale(s: crate::api::settings::ScaleMode) -> IntensityScale {
    match s {
        crate::api::settings::ScaleMode::Linear => IntensityScale::Linear,
        crate::api::settings::ScaleMode::Sqrt => IntensityScale::Sqrt,
        crate::api::settings::ScaleMode::Cbrt => IntensityScale::Cbrt,
        crate::api::settings::ScaleMode::Log => IntensityScale::Log,
    }
}
