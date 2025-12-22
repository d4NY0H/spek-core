//! Output types for spek-core.
//!
//! This module defines the final, public result returned by the core.
//! All data here is stable API surface.

use crate::render::ImageBuffer;

/// Spectrogram generation result.
///
/// The image ALWAYS includes the legend.
#[derive(Debug, Clone)]
pub struct SpectrogramResult {
    /// Final rendered RGBA image buffer (with legend)
    pub image: ImageBuffer,

    /// Duration of the input audio in seconds
    pub duration_seconds: f64,

    /// Sample rate of the input audio in Hz
    pub sample_rate: u32,

    /// Number of channels in the input audio
    pub channels: u32,
}
