//! Audio input abstraction for spek-core.
//!
//! In spek-core, "audio" means:
//! - invoking an external backend (ffmpeg)
//! - obtaining a rendered spectrogram image
//! - collecting minimal audio metadata
//!
//! spek-core does NOT decode PCM,
//! does NOT perform FFT,
//! and does NOT do DSP of any kind.

use crate::render::ImageBuffer;

/// Audio metadata required for legend rendering.
///
/// This metadata is informational only and
/// mirrors what Spek / spek-rs exposes.
#[derive(Debug, Clone)]
pub struct AudioMetadata {
    /// Sample rate in Hz (e.g. 44100)
    pub sample_rate: u32,

    /// Number of channels (1 = mono, 2 = stereo, ...)
    pub channels: u16,

    /// Total duration in seconds
    pub duration_sec: f64,

    /// Bit depth of the original source, if known
    pub bit_depth: Option<u16>,
}

/// Result of an audio source invocation.
///
/// This is the ONLY data spek-core needs
/// from an audio backend.
#[derive(Debug)]
pub struct SpectrogramSource {
    /// Rendered spectrogram image (RGBA, opaque)
    pub image: ImageBuffer,

    /// Audio metadata for legend / UI
    pub meta: AudioMetadata,
}

/// Abstract audio source.
///
/// Implementations:
/// - ffmpeg CLI backend (Linux, Colab)
/// - later: platform-specific wrappers
///
/// IMPORTANT:
/// Implementations MUST render the spectrogram themselves.
/// spek-core never touches PCM or FFT data.
pub trait AudioSource {
    /// Generate a spectrogram image and metadata.
    fn load(&self) -> Result<SpectrogramSource, AudioError>;
}

/// Audio loading / rendering errors.
#[derive(Debug)]
pub enum AudioError {
    /// Input format not supported by backend
    UnsupportedFormat,

    /// ffmpeg or backend failed
    DecodeFailed,

    /// I/O failure
    IoError,

    /// Operation cancelled by caller
    Cancelled,
}
