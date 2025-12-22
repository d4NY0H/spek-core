//! FFmpeg-based audio decoder for spek-core.
//!
//! This module provides a concrete AudioSource implementation
//! using ffmpeg as a decoding backend.
//!
//! It is intended for:
//! - Linux (CLI / Colab)
//! - CI environments
//!
//! This module is still HEADLESS and UI-agnostic.

use std::path::PathBuf;

use crate::audio::{AudioBuffer, AudioError, AudioMetadata, AudioSource};

/// FFmpeg-backed audio source.
///
/// This struct represents an audio file that will be decoded
/// entirely into normalized PCM float samples.
#[derive(Debug, Clone)]
pub struct FfmpegAudioSource {
    /// Path to the input audio file
    pub path: PathBuf,
}

impl FfmpegAudioSource {
    /// Create a new FFmpeg audio source from a file path.
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }
}

impl AudioSource for FfmpegAudioSource {
    fn load(&self) -> Result<AudioBuffer<'_>, AudioError> {
        // NOTE:
        // Actual ffmpeg invocation will be implemented later.
        //
        // The final behavior will be:
        // - invoke ffmpeg
        // - decode audio to f32 PCM
        // - normalize to [-1.0, 1.0]
        // - collect metadata (sample rate, channels, bit depth)
        //
        // For now, this is a structural stub.

        Err(AudioError::UnsupportedFormat)
    }
}
