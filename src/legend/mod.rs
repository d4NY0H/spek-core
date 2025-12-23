//! Legend rendering for spek-core.
//!
//! This module defines the semantic model for spectrogram legends.
//! It generates axis labels, scales, and metadata overlays.
//!
//! The legend is ALWAYS rendered and never optional.

use crate::audio::AudioMetadata;

pub mod overlay;
pub mod simple;

/// Legend layout margins (in pixels).
#[derive(Debug, Copy, Clone)]
pub struct LegendMargins {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}

/// Legend configuration.
///
/// Controls visual density and tick layout.
/// All fields are mandatory.
/// There is NO disable flag by design.
#[derive(Debug, Clone)]
pub struct LegendSettings {
    /// Font size in pixels (logical size, renderer-defined)
    pub font_size: u32,

    /// Number of frequency ticks
    pub freq_ticks: usize,

    /// Number of time ticks
    pub time_ticks: usize,

    /// Number of dB ticks
    pub db_ticks: usize,
}

/// Context required to generate a legend.
///
/// This struct contains ALL semantic information
/// required to describe legend contents.
///
/// It is:
/// - backend-agnostic
/// - rendering-independent
/// - deterministic
///
/// Population of this struct is the responsibility of the API layer,
/// NOT the legend renderer itself.
#[derive(Debug, Clone)]
pub struct LegendContext {
    /// Audio metadata (sample rate, channels, bit depth)
    pub audio: AudioMetadata,

    /// Total duration in seconds
    pub duration_sec: f64,

    /// Minimum dBFS shown (e.g. -120.0)
    pub min_db: f32,

    /// Maximum dBFS shown (usually 0.0)
    pub max_db: f32,

    /// Display name of the input audio file.
    ///
    /// This value is optional and purely informational.
    /// It may be truncated or omitted by the legend renderer.
    ///
    /// Typical source:
    /// - CLI argument
    /// - UI layer
    pub file_name: Option<String>,

    /// Application / core version string.
    ///
    /// Example:
    /// - "spek-core 0.1.0"
    ///
    /// This is NOT generated automatically.
    /// The caller decides whether to expose version info.
    pub app_version: Option<String>,
}

/// Output commands produced by the legend system.
///
/// These commands are backend-agnostic and can be
/// rendered by any text / vector backend.
#[derive(Debug, Clone)]
pub enum LegendCommand {
    /// Draw text at pixel position.
    Text {
        x: u32,
        y: u32,
        content: String,
    },

    /// Draw a straight line.
    Line {
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
    },

    /// Draw a vertical dBFS gradient bar (Spek-style).
    ///
    /// This is semantically NOT a line.
    /// It represents a continuous color scale from max → min dBFS.
    ///
    /// - `y_top` corresponds to max dBFS (bright)
    /// - `y_bottom` corresponds to min dBFS (dark)
    DbfsGradient {
        x: u32,
        y_top: u32,
        y_bottom: u32,
    },
}

/// Legend renderer interface.
///
/// Produces a sequence of draw commands describing:
/// - axes
/// - ticks
/// - labels
/// - metadata
///
/// The renderer:
/// - MUST NOT modify context
/// - MUST NOT access audio or render state directly
pub trait LegendRenderer {
    fn generate(
        &self,
        settings: &LegendSettings,
        context: &LegendContext,
        margins: LegendMargins,
        image_width: u32,
        image_height: u32,
    ) -> Vec<LegendCommand>;
}
