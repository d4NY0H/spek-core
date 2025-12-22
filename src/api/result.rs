//! Output types for spek-core.

/// Raw RGBA image buffer.
#[derive(Debug, Clone)]
pub struct ImageBuffer {
    /// Image width in pixels
    pub width: u32,

    /// Image height in pixels
    pub height: u32,

    /// RGBA pixel data (width * height * 4)
    pub data: Vec<u8>,
}

/// Spectrogram generation result.
#[derive(Debug, Clone)]
pub struct SpectrogramResult {
    /// Final rendered image (including legend)
    pub image: ImageBuffer,

    /// Duration of the input audio in seconds
    pub duration_seconds: f64,

    /// Sample rate of the input audio
    pub sample_rate: u32,

    /// Number of channels in input audio
    pub channels: u32,
}
