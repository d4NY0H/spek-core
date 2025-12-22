//! Public spectrogram generation API.
//!
//! This module defines the single, stable entry point into spek-core.
//! It orchestrates the full pipeline:
//!
//! audio → analysis → render → legend → result
//!
//! No UI logic, no platform-specific code.

use crate::api::result::SpectrogramResult;
use crate::api::settings::SpectrogramSettings;

use crate::analysis::{Analyzer, AnalysisError};
use crate::audio::{AudioError, AudioSource};
use crate::render::RenderError;

/// Generate a spectrogram image including legend.
///
/// This is the ONLY public entry point of spek-core.
///
/// One call → one deterministic result.
///
/// The function is:
/// - synchronous
/// - side-effect free
/// - deterministic
///
/// # Errors
///
/// Returns an error if:
/// - the input audio cannot be opened
/// - decoding fails
/// - analysis parameters are invalid
/// - rendering fails
pub fn generate_spectrogram(
    input_path: &str,
    settings: &SpectrogramSettings,
) -> Result<SpectrogramResult, GenerateError> {
    // ---------------------------------------------------------------------
    // 1. Create audio source (backend chosen by settings)
    // ---------------------------------------------------------------------
    let audio_source = settings
        .audio_backend
        .create_source(input_path)
        .map_err(|_| GenerateError::AudioOpenFailed)?;

    // ---------------------------------------------------------------------
    // 2. Decode audio into PCM buffer
    // ---------------------------------------------------------------------
    let audio = audio_source.load().map_err(map_audio_error)?;

    // ---------------------------------------------------------------------
    // 3. Run signal analysis
    // ---------------------------------------------------------------------
    let analyzer = settings.analysis.create_analyzer();
    let spectrograms = analyzer
        .analyze(&audio, &settings.analysis)
        .map_err(map_analysis_error)?;

    // ---------------------------------------------------------------------
    // 4. Render spectrogram pixels
    // ---------------------------------------------------------------------
    let renderer = settings.render.create_renderer();
    let image = renderer
        .render(&spectrograms, &settings.render)
        .map_err(map_render_error)?;

    // ---------------------------------------------------------------------
    // 5. Render legend overlay
    // ---------------------------------------------------------------------
    let legend = settings.legend.create_legend(
        &audio.meta,
        &settings.analysis,
        image.width,
        image.height,
    );

    // ---------------------------------------------------------------------
    // 6. Assemble final result
    // ---------------------------------------------------------------------
    Ok(SpectrogramResult {
        image,
        legend,
    })
}

// -------------------------------------------------------------------------
// Error mapping helpers
// -------------------------------------------------------------------------

fn map_audio_error(err: AudioError) -> GenerateError {
    match err {
        AudioError::UnsupportedFormat => GenerateError::DecodeFailed,
        AudioError::DecodeFailed => GenerateError::DecodeFailed,
        AudioError::IoError => GenerateError::AudioOpenFailed,
        AudioError::Cancelled => GenerateError::Cancelled,
    }
}

fn map_analysis_error(_err: AnalysisError) -> GenerateError {
    GenerateError::InvalidSettings
}

fn map_render_error(_err: RenderError) -> GenerateError {
    GenerateError::RenderFailed
}

/// Public error type for spectrogram generation.
///
/// This type is stable and backend-agnostic.
#[derive(Debug)]
pub enum GenerateError {
    /// Input file could not be opened or read
    AudioOpenFailed,

    /// Audio decoding failed
    DecodeFailed,

    /// Analysis parameters are invalid
    InvalidSettings,

    /// Rendering failed
    RenderFailed,

    /// Operation was cancelled
    Cancelled,
}
