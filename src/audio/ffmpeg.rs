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
        // 2. Decode audio to f32 PCM via ffmpeg (MINIMAL + SAFE)
        // -------------------------------------------------------------
        let decode = Command::new("ffmpeg")
            .args([
                "-nostdin",
                "-v",
                "error",
                "-vn",
                "-sn",
                "-dn",
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
        // 3. Convert raw bytes → f32 samples (CLAMPED)
        // -------------------------------------------------------------
        let bytes = decode.stdout;
        if bytes.len() % 4 != 0 {
            return Err(AudioError::DecodeFailed);
        }

        let mut samples: Vec<f32> = Vec::with_capacity(bytes.len() / 4);

        for chunk in bytes.chunks_exact(4) {
            let mut v = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);

            // Critical safety:
            if !v.is_finite() {
                v = 0.0;
            } else {
                v = v.clamp(-1.0, 1.0);
            }

            samples.push(v);
        }

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
            // Intentional leak: AudioBuffer is immutable & lives for program lifetime
            samples: Box::leak(samples.into_boxed_slice()),
            meta,
        })
    }
}
