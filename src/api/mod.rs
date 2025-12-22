//! spek-core public API
//!
//! This module defines the ONLY supported entry points into the core.
//! No UI, no platform logic, no side effects.

// ---------------------------------------------------------------------
// Submodules
// ---------------------------------------------------------------------

pub mod generate;
pub mod settings;
pub mod result;

// ---------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------

use crate::audio::AudioSource;

use generate::GenerateError;
use settings::SpectrogramSettings;
use result::SpectrogramResult;

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
            GenerateError::DecodeFailed => SpekError::DecodeError,
            GenerateError::AnalysisFailed => SpekError::AnalysisError,
            GenerateError::RenderFailed => SpekError::RenderError,
        })
}
