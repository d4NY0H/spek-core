//! Simple deterministic legend renderer for spek-core.
//!
//! This renderer generates axis lines and text labels
//! as abstract drawing commands.
//!
//! It does NOT rasterize fonts and does NOT touch pixels.

use crate::legend::{
    LegendCommand, LegendContext, LegendMargins, LegendRenderer, LegendSettings,
};

/// Default legend renderer (Spek-style).
///
/// Produces:
/// - Time axis (bottom)
/// - Frequency axis (left)
/// - dBFS scale (right)
/// - Title line (top)
///
/// All output is deterministic and resolution-independent.
pub struct SimpleLegendRenderer;

impl SimpleLegendRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl LegendRenderer for SimpleLegendRenderer {
    fn generate(
        &self,
        settings: &LegendSettings,
        context: &LegendContext,
        margins: LegendMargins,
        image_width: u32,
        image_height: u32,
    ) -> Vec<LegendCommand> {
        let mut cmds = Vec::new();

        let left = margins.left;
        let right = image_width - margins.right;
        let top = margins.top;
        let bottom = image_height - margins.bottom;

        // -----------------------------------------------------------------
        // Axis lines
        // -----------------------------------------------------------------
        cmds.push(line(left, top, left, bottom));       // Frequency axis
        cmds.push(line(left, bottom, right, bottom));   // Time axis
        cmds.push(line(right, top, right, bottom));     // dB axis

        // -----------------------------------------------------------------
        // Title (top)
        // -----------------------------------------------------------------
        cmds.push(text(
            left,
            top.saturating_sub(settings.font_size + 8),
            "Spectrogram",
        ));

        // -----------------------------------------------------------------
        // Time axis labels
        // -----------------------------------------------------------------
        for i in 0..=settings.time_ticks {
            let t = i as f64 / settings.time_ticks as f64;
            let x = left + ((right - left) as f64 * t) as u32;
            let seconds = context.duration_sec * t;

            cmds.push(line(x, bottom, x, bottom + 6));
            cmds.push(text(
                x.saturating_sub(12),
                bottom + 10,
                &format!("{:.1}s", seconds),
            ));
        }

        // -----------------------------------------------------------------
        // Frequency axis labels (linear, Hz → kHz)
        // -----------------------------------------------------------------
        let nyquist = context.audio.sample_rate as f64 / 2.0;

        for i in 0..=settings.freq_ticks {
            let f = i as f64 / settings.freq_ticks as f64;
            let y = bottom - ((bottom - top) as f64 * f) as u32;
            let hz = nyquist * f;

            cmds.push(line(left - 6, y, left, y));
            cmds.push(text(
                4,
                y.saturating_sub(settings.font_size / 2),
                &format!("{:.1} kHz", hz / 1000.0),
            ));
        }

        // -----------------------------------------------------------------
        // dBFS scale (right)
        // -----------------------------------------------------------------
        let db_range = context.max_db - context.min_db;

        for i in 0..=settings.db_ticks {
            let t = i as f32 / settings.db_ticks as f32;
            let y = bottom - ((bottom - top) as f32 * t) as u32;
            let db = context.min_db + db_range * t;

            cmds.push(line(right, y, right + 6, y));
            cmds.push(text(
                right + 10,
                y.saturating_sub(settings.font_size / 2),
                &format!("{:.0} dB", db),
            ));
        }

        cmds
    }
}

// -------------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------------

#[inline]
fn line(x1: u32, y1: u32, x2: u32, y2: u32) -> LegendCommand {
    LegendCommand::Line { x1, y1, x2, y2 }
}

#[inline]
fn text(x: u32, y: u32, content: &str) -> LegendCommand {
    LegendCommand::Text {
        x,
        y,
        content: content.to_string(),
    }
}
