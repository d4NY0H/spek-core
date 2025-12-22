//! spek-core public API
//!
//! This module defines the ONLY supported entry points into the core.
//! No UI, no platform logic, no side effects.

use crate::audio::AudioSource;
use crate::render::ImageBuffer;

use crate::api::generate;
use crate::api::settings::SpectrogramSettings;
use crate::api::result::SpectrogramResult;

/// Errors returned by spek-core.
///
/// This type is intentionally small and stable.
#[derive(Debug)]
pub enum SpekError {
    InvalidInput,
    DecodeError,
    AnalysisError,
    RenderError,
    Cancelled,
}

/// Generate a spectrogram image with a mandatory legend.
///
/// This function is:
/// - deterministic
/// - blocking
/// - side-effect free
///
/// One call = one image.
///
/// This is a thin wrapper around `api::generate::generate_spectrogram`.
pub fn generate_spectrogram(
    source: &dyn AudioSource,
    settings: &SpectrogramSettings,
) -> Result<SpectrogramResult, SpekError> {
    generate::generate_spectrogram(source, settings)
        .map_err(|e| match e {
            generate::GenerateError::DecodeFailed => SpekError::DecodeError,
            generate::GenerateError::AnalysisFailed => SpekError::AnalysisError,
            generate::GenerateError::RenderFailed => SpekError::RenderError,
        })
}
