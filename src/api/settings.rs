//! Public configuration types for spek-core.

/// Spectrogram scaling mode.
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
    Combined,
    Split,
}

/// Spectrogram generation settings.
#[derive(Debug, Clone)]
pub struct SpectrogramSettings {
    /// FFT size (e.g. 1024, 2048)
    pub fft_size: usize,

    /// Hop size (overlap control)
    pub hop_size: usize,

    /// Window function
    pub window: WindowFunction,

    /// Channel handling
    pub channels: ChannelMode,

    /// Minimum dBFS (e.g. -120.0)
    pub min_db: f32,

    /// Maximum dBFS (usually 0.0)
    pub max_db: f32,

    /// Intensity scaling
    pub scale: ScaleMode,
}

/// Render output settings.
#[derive(Debug, Clone)]
pub struct RenderSettings {
    /// Output image width in pixels
    pub width: u32,

    /// Output image height in pixels
    pub height: u32,
}

/// Complete spek-core configuration.
#[derive(Debug, Clone)]
pub struct SpekSettings {
    pub spectrogram: SpectrogramSettings,
    pub render: RenderSettings,
}
