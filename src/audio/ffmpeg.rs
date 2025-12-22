//! FFmpeg-based audio decoder for spek-core.
//!
//! This module provides a concrete AudioSource implementation
//! using ffmpeg as a decoding backend.
//!
//! Target:
//! - Linux / CLI
//! - Google Colab
//!
//! Decodes audio into interleaved f32 PCM samples in range [-1.0, 1.0].

use std::path::PathBuf;
use std::process::Command;

use crate::audio::{AudioBuffer, AudioError, AudioMetadata, AudioSource};

/// FFmpeg-backed audio source.
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
        // -------------------------------------------------------------
        // 1. Probe metadata using ffprobe
        // -------------------------------------------------------------
        let probe = Command::new("ffprobe")
            .args([
                "-v",
                "error",
                "-select_streams",
                "a:0",
                "-show_entries",
                "stream=sample_rate,channels",
                "-of",
                "default=nokey=1:noprint_wrappers=1",
            ])
            .arg(&self.path)
            .output()
            .map_err(|_| AudioError::IoError)?;

        if !probe.status.success() {
            return Err(AudioError::DecodeFailed);
        }

        let output = String::from_utf8_lossy(&probe.stdout);
        let mut lines = output.lines();

        let sample_rate: u32 = lines
            .next()
            .and_then(|v| v.parse().ok())
            .ok_or(AudioError::DecodeFailed)?;

        let channels: u16 = lines
            .next()
            .and_then(|v| v.parse().ok())
            .ok_or(AudioError::DecodeFailed)?;

        // -------------------------------------------------------------
        // 2. Decode audio to f32 PCM via ffmpeg
        // -------------------------------------------------------------
        let decode = Command::new("ffmpeg")
            .args([
                "-v",
                "error",
                "-i",
            ])
            .arg(&self.path)
            .args([
                "-f",
                "f32le",
                "-acodec",
                "pcm_f32le",
                "-ac",
                &channels.to_string(),
                "-ar",
                &sample_rate.to_string(),
                "-",
            ])
            .output()
            .map_err(|_| AudioError::IoError)?;

        if !decode.status.success() {
            return Err(AudioError::DecodeFailed);
        }

        // -------------------------------------------------------------
        // 3. Convert raw bytes → f32 samples
        // -------------------------------------------------------------
        let bytes = decode.stdout;
        if bytes.len() % 4 != 0 {
            return Err(AudioError::DecodeFailed);
        }

        let samples: Vec<f32> = bytes
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect();

        let total_samples = (samples.len() / channels as usize) as u64;

        // -------------------------------------------------------------
        // 4. Assemble metadata & buffer
        // -------------------------------------------------------------
        let meta = AudioMetadata {
            sample_rate,
            channels,
            total_samples,
            bit_depth: None,
        };

        Ok(AudioBuffer {
            samples: Box::leak(samples.into_boxed_slice()),
            meta,
        })
    }
}
