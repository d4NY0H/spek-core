//! Legend rendering for spek-core.
//!
//! This module generates axis labels, scales, and metadata overlays.
//! The legend is always rendered and never optional.

use crate::audio::AudioInfo;

/// Legend layout margins (in pixels).
#[derive(Debug, Copy, Clone)]
pub struct LegendMargins {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}

/// Legend configuration.
#[derive(Debug, Clone)]
pub struct LegendSettings {
    /// Enable legend rendering (always true in spek-core)
    pub enabled: bool,

    /// Font size in pixels
    pub font_size: u32,

    /// Number of frequency ticks
    pub freq_ticks: usize,

    /// Number of time ticks
    pub time_ticks: usize,

    /// Number of dB ticks
    pub db_ticks: usize,
}

/// Information required to draw a legend.
#[derive(Debug, Clone)]
pub struct LegendContext {
    /// Audio metadata
    pub audio: AudioInfo,

    /// Duration in seconds
    pub duration_sec: f64,

    /// Minimum dBFS (e.g. -120.0)
    pub min_db: f32,

    /// Maximum dBFS (usually 0.0)
    pub max_db: f32,
}

/// Output commands produced by the legend system.
#[derive(Debug, Clone)]
pub enum LegendCommand {
    Text {
        x: u32,
        y: u32,
        content: String,
    },
    Line {
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
    },
}

/// Legend renderer interface.
pub trait LegendRenderer {
    /// Generate legend drawing commands.
    fn generate(
        &self,
        settings: &LegendSettings,
        context: &LegendContext,
        margins: LegendMargins,
        image_width: u32,
        image_height: u32,
    ) -> Vec<LegendCommand>;
}
