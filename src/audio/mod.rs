//! Audio input abstraction for spek-core.
//!
//! This module defines the platform-agnostic audio interface.
//! It does NOT decode audio, perform DSP, or depend on ffmpeg
//! or any specific backend.

/// Audio metadata required by the signal pipeline.
#[derive(Debug, Clone)]
pub struct AudioMetadata {
    /// Sample rate in Hz (e.g. 44100)
    pub sample_rate: u32,

    /// Number of channels (1 = mono, 2 = stereo, ...)
    pub channels: u16,

    /// Total number of samples per channel
    pub total_samples: u64,

    /// Bit depth of the original source, if known
    pub bit_depth: Option<u16>,
}

/// PCM audio buffer in normalized floating-point format.
///
/// All samples must be in the range [-1.0, 1.0].
/// Samples are interleaved: L, R, L, R, ...
#[derive(Debug)]
pub struct AudioBuffer {
    /// Interleaved PCM samples
    pub samples: Vec<f32>,

    /// Associated metadata
    pub meta: AudioMetadata,
}

/// Abstract audio source.
///
/// This trait will later be implemented by:
/// - ffmpeg-based loaders (Linux)
/// - JNI bridges (Android)
/// - Swift / C ABI bridges (iPadOS)
///
/// spek-core only depends on THIS interface.
pub trait AudioSource {
    /// Load the entire audio stream into memory.
    ///
    /// The returned buffer must be complete and immutable.
    fn load(&self) -> Result<AudioBuffer, AudioError>;
}

/// Audio loading errors.
///
/// This is intentionally small and backend-agnostic.
#[derive(Debug)]
pub enum AudioError {
    UnsupportedFormat,
    DecodeFailed,
    IoError,
    Cancelled,
}
