//! spek-core public API
//!
//! This module defines the ONLY supported entry points into the core.
//! No UI, no platform logic, no side effects.

use crate::audio::AudioSource;
use crate::render::ImageBuffer;

/// High-level settings controlling spectrogram generation.
///
/// These settings are platform-agnostic and deterministic.
#[derive(Debug, Clone)]
pub struct SpectrogramSettings {
    pub fft_size: usize,
    pub hop_size: usize,
    pub min_db: f32,
    pub max_db: f32,
    pub scale: IntensityScale,
    pub split_channels: bool,
}

/// Intensity scaling applied after dBFS mapping.
#[derive(Debug, Clone, Copy)]
pub enum IntensityScale {
    Linear,
    Sqrt,
    Cbrt,
    Log,
}

/// Errors returned by spek-core.
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
/// NOTE:
/// The concrete pipeline wiring lives in `api::generate`.
pub fn generate_spectrogram(
    _source: &dyn AudioSource,
    _settings: SpectrogramSettings,
) -> Result<ImageBuffer, SpekError> {
    // This function is intentionally a thin wrapper.
    // The real implementation is provided by api::generate.
    unimplemented!("spek-core API entry point")
}
