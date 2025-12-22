//! Signal analysis data structures for spek-core.
//!
//! This module defines the data model for numerical
//! spectrograms. It contains NO DSP logic and NO rendering.

/// Scaling applied to intensity values.
#[derive(Debug, Copy, Clone)]
pub enum IntensityScale {
    Linear,
    Sqrt,
    Cbrt,
    Log,
    Power(f32),
}

/// Window function used during FFT.
#[derive(Debug, Copy, Clone)]
pub enum WindowFunction {
    Rectangular,
    Hann,
    Hamming,
    Blackman,
    Nuttall,
    Kaiser,
    FlatTop,
}

/// Parameters controlling the signal analysis stage.
#[derive(Debug, Clone)]
pub struct AnalysisSettings {
    /// FFT window size (e.g. 1024, 2048)
    pub fft_size: usize,

    /// Hop size between windows
    pub hop_size: usize,

    /// Window function
    pub window: WindowFunction,

    /// Intensity scaling after dB mapping
    pub scale: IntensityScale,

    /// Minimum dBFS floor (e.g. -120.0)
    pub min_db: f32,
}

/// Numerical spectrogram for a single channel.
///
/// Data layout:
/// data[freq_bin][time_bin] -> intensity (0.0..1.0)
#[derive(Debug)]
pub struct Spectrogram {
    pub freq_bins: usize,
    pub time_bins: usize,
    pub data: Vec<Vec<f32>>,
}

/// Multi-channel spectrogram output.
///
/// - Combined mode: one entry
/// - Split mode: one entry per channel
#[derive(Debug)]
pub struct SpectrogramSet {
    pub channels: Vec<Spectrogram>,
}

/// Signal analysis interface.
///
/// This trait performs the complete DSP pipeline:
/// PCM -> FFT -> dB -> scaling -> spectrogram
pub trait Analyzer {
    fn analyze(
        &self,
        audio: &crate::audio::AudioBuffer,
        settings: &AnalysisSettings,
    ) -> Result<SpectrogramSet, AnalysisError>;
}

/// Analysis errors.
#[derive(Debug)]
pub enum AnalysisError {
    InvalidParameters,
    ProcessingFailed,
    Cancelled,
}
