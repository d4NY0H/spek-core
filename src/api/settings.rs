//! Public configuration types for spek-core.
//!
//! These types define the complete, stable input contract
//! for spectrogram generation.
//!
//! No UI-specific or platform-specific settings are allowed here.

/// Intensity scaling mode applied after dBFS mapping.
#[derive(Debug, Copy, Clone)]
pub enum ScaleMode {
    Linear,
    Sqrt,
    Cbrt,
    Log,
}

/// FFT window function.
#[derive(Debug, Copy, Clone)]
pub enum WindowFunction {
    Rectangular,
    Hann,
    Hamming,
    Blackman,
}

/// Channel processing mode.
#[derive(Debug, Copy, Clone)]
pub enum ChannelMode {
    /// All channels combined into a single spectrogram
    Combined,

    /// One spectrogram per channel
    Split,
}

/// Numerical spectrogram generation settings.
///
/// These parameters control the signal analysis stage.
#[derive(Debug, Clone)]
pub struct SpectrogramSettings {
    /// FFT size (e.g. 1024, 2048, 4096)
    pub fft_size: usize,

    /// Hop size between FFT windows
    pub hop_size: usize,

    /// Window function
    pub window: WindowFunction,

    /// Channel handling mode
    pub channels: ChannelMode,

    /// Minimum dBFS floor (e.g. -120.0)
    pub min_db: f32,

    /// Maximum dBFS ceiling (typically 0.0)
    pub max_db: f32,

    /// Intensity scaling mode
    pub scale: ScaleMode,
}

/// Final output image configuration.
///
/// This controls the pixel dimensions of the rendered image.
/// The legend is always included automatically.
#[derive(Debug, Clone)]
pub struct RenderSettings {
    /// Output image width in pixels
    pub width: u32,

    /// Output image height in pixels
    pub height: u32,
}

/// Complete spek-core configuration.
///
/// This is the single settings object accepted by the public API.
#[derive(Debug, Clone)]
pub struct SpekSettings {
    /// Spectrogram analysis settings
    pub spectrogram: SpectrogramSettings,

    /// Render output settings
    pub render: RenderSettings,
}
