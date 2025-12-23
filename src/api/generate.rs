//! Public spectrogram generation API.
//!
//! This module defines the single, stable entry point into spek-core.
//!
//! spek-core follows the Spek / spek-rs model strictly:
//! - ffmpeg renders the spectrogram image
//! - spek-core only overlays legend information
//!
//! No DSP, no FFT, no color mapping happens here.

use crate::api::result::SpectrogramResult;
use crate::api::settings::SpekSettings;

use crate::audio::{AudioSource, AudioError};
use crate::legend::{
    LegendRenderer, LegendContext, LegendSettings, LegendMargins,
};

/// Generate a spectrogram image including legend.
///
/// This is the ONLY public entry point of spek-core.
///
/// One call → one deterministic result.
pub fn generate_spectrogram(
    source: &dyn AudioSource,
    legend: &dyn LegendRenderer,
    settings: &SpekSettings,
) -> Result<SpectrogramResult, GenerateError> {
    // ---------------------------------------------------------------------
    // 1. Load spectrogram image + metadata from audio source
    // ---------------------------------------------------------------------
    let source_result = source
        .load()
        .map_err(map_audio_error)?;

    let mut image = source_result.image;
    let meta = source_result.meta;

    // ---------------------------------------------------------------------
    // 2. Build legend context
    // ---------------------------------------------------------------------
    let legend_context = LegendContext {
        audio: meta.clone(),
        duration_sec: meta.duration_sec,
        min_db: settings.spectrogram.min_db,
        max_db: settings.spectrogram.max_db,

        // Optional informational fields
        file_name: None,
        app_version: Some(format!(
            "spek-core {}",
            env!("CARGO_PKG_VERSION")
        )),
    };

    // ---------------------------------------------------------------------
    // 3. Generate legend commands
    // ---------------------------------------------------------------------
    let legend_commands = legend.generate(
        &LegendSettings {
            font_size: 14,
            freq_ticks: 10,
            time_ticks: 10,
            db_ticks: 6,
        },
        &legend_context,
        LegendMargins {
            left: 80,
            right: 100,
            top: 60,
            bottom: 60,
        },
        image.width as u32,
        image.height as u32,
    );

    // ---------------------------------------------------------------------
    // 4. Apply legend overlay
    // ---------------------------------------------------------------------
    crate::legend::overlay::apply_legend_overlay(
        &mut image,
        &legend_commands,
    );

    // ---------------------------------------------------------------------
    // 5. Assemble result
    // ---------------------------------------------------------------------
    Ok(SpectrogramResult {
        image,
        duration_seconds: meta.duration_sec,
        sample_rate: meta.sample_rate,
        channels: meta.channels as u32,
    })
}

/// Public error type for spectrogram generation.
#[derive(Debug)]
pub enum GenerateError {
    DecodeFailed,
}

/// Map backend audio errors to public API errors.
fn map_audio_error(err: AudioError) -> GenerateError {
    match err {
        AudioError::UnsupportedFormat => GenerateError::DecodeFailed,
        AudioError::DecodeFailed => GenerateError::DecodeFailed,
        AudioError::IoError => GenerateError::DecodeFailed,
        AudioError::Cancelled => GenerateError::DecodeFailed,
    }
}
