//! Spectrogram rendering model for spek-core.
//!
//! NOTE:
//! This module is kept temporarily for API compatibility.
//! All actual spectrogram rendering is delegated to ffmpeg.

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
