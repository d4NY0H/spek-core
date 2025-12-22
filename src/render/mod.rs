//! Spectrogram rendering model for spek-core.
//!
//! Converts numerical spectrogram data into a pixel buffer.
//! This module does NOT handle legends, text, fonts, or metadata overlays.

use crate::analysis::SpectrogramSet;

/// Orientation of the spectrogram.
#[derive(Debug, Copy, Clone)]
pub enum Orientation {
    /// Frequency on Y axis, time on X axis
    Vertical,

    /// Frequency on X axis, time on Y axis
    Horizontal,
}

/// Channel layout strategy.
#[derive(Debug, Copy, Clone)]
pub enum ChannelLayout {
    /// All channels combined into a single spectrogram
    Combined,

    /// Each channel rendered separately
    Split,
}

/// Rendering parameters.
///
/// These parameters fully define how numerical spectrogram
/// data is mapped to pixel space.
#[derive(Debug, Clone)]
pub struct RenderSettings {
    /// Output image width in pixels
    pub width: usize,

    /// Output image height in pixels
    pub height: usize,

    /// Spectrogram orientation
    pub orientation: Orientation,

    /// Channel layout strategy
    pub channels: ChannelLayout,
}

/// RGBA8 image buffer (row-major).
///
/// Layout:
/// data.len() == width * height * 4
#[derive(Debug, Clone)]
pub struct ImageBuffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

/// Renderer interface.
///
/// Converts numerical spectrogram data into a raw pixel buffer.
///
/// Color mapping is handled internally by the renderer implementation.
pub trait Renderer {
    fn render(
        &self,
        spectrograms: &SpectrogramSet,
        settings: &RenderSettings,
    ) -> Result<ImageBuffer, RenderError>;
}

/// Rendering errors.
#[derive(Debug)]
pub enum RenderError {
    /// Output dimensions are invalid or inconsistent
    InvalidDimensions,

    /// Internal rendering failure
    Failed,
}

/// Helper: write a single RGBA pixel into the buffer.
#[inline]
pub fn put_pixel(
    image: &mut ImageBuffer,
    x: usize,
    y: usize,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    if x >= image.width || y >= image.height {
        return;
    }

    let idx = (y * image.width + x) * 4;
    if idx + 3 < image.data.len() {
        image.data[idx] = r;
        image.data[idx + 1] = g;
        image.data[idx + 2] = b;
        image.data[idx + 3] = a;
    }
}
