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
/// - processing is cancelled
pub fn generate_spectrogram(
    input_path: &str,
    settings: &SpectrogramSettings,
) -> Result<SpectrogramResult, GenerateError> {
    // Implementation will be wired step by step:
    //
    // 1. Open audio source
    // 2. Decode audio into PCM buffer
    // 3. Run signal analysis
    // 4. Render spectrogram pixels
    // 5. Render legend overlay
    // 6. Assemble final result

    Err(GenerateError::NotImplemented)
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

    /// Placeholder during development
    NotImplemented,
}
