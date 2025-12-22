//! Legend rendering for spek-core.
//!
//! This module generates axis labels, scales, and metadata overlays.
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
/// All fields are mandatory.
/// There is NO disable flag by design.
#[derive(Debug, Clone)]
pub struct LegendSettings {
    /// Font size in pixels
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
/// required to describe the legend contents.
/// It is backend-agnostic and rendering-independent.
#[derive(Debug, Clone)]
pub struct LegendContext {
    /// Audio metadata
    pub audio: AudioMetadata,

    /// Duration in seconds
    pub duration_sec: f64,

    /// Minimum dBFS (e.g. -120.0)
    pub min_db: f32,

    /// Maximum dBFS (usually 0.0)
    pub max_db: f32,

    /// Display name of the input audio file
    ///
    /// This is purely informational and may be truncated
    /// by the legend renderer.
    pub file_name: Option<String>,

    /// Application / core version string
    ///
    /// Example: "spek-core 0.1.0"
    pub app_version: Option<String>,
}

/// Output commands produced by the legend system.
///
/// These commands are backend-agnostic and can be
/// rendered by any text / vector backend.
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
///
/// Produces a sequence of draw commands describing
/// axes, ticks, labels, and metadata.
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
