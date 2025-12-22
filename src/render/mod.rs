//! Spectrogram rendering model for spek-core.
//!
//! Converts numerical spectrogram data into a pixel buffer.
//! This module does NOT handle legends, text, or fonts.

use crate::analysis::Spectrogram;

/// Orientation of the spectrogram.
#[derive(Debug, Copy, Clone)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

/// Channel layout strategy.
#[derive(Debug, Copy, Clone)]
pub enum ChannelLayout {
    Combined,
    Split,
}

/// Rendering parameters.
#[derive(Debug, Clone)]
pub struct RenderSettings {
    /// Output image width in pixels
    pub width: usize,

    /// Output image height in pixels
    pub height: usize,

    /// Spectrogram orientation
    pub orientation: Orientation,

    /// Channel layout
    pub channels: ChannelLayout,
}

/// RGBA pixel buffer.
///
/// Layout: row-major, 4 bytes per pixel.
#[derive(Debug)]
pub struct ImageBuffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

/// Render interface.
///
/// Converts spectrogram data into a pixel buffer.
/// Color mapping is handled elsewhere.
pub trait Renderer {
    fn render(
        &self,
        spectrogram: &Spectrogram,
        settings: &RenderSettings,
    ) -> Result<ImageBuffer, RenderError>;
}

/// Rendering errors.
#[derive(Debug)]
pub enum RenderError {
    InvalidDimensions,
    Failed,
}
