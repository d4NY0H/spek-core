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
/// - Optional file / metadata header (top)
/// - Time axis (bottom) with labels + top ticks without labels
/// - Frequency axis (left + right ticks, labels left only)
/// - dBFS scale (right)
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
        // Header (top metadata) – OPTIONAL
        // -----------------------------------------------------------------

        let header_y = top.saturating_sub(settings.font_size + 8);

        if let Some(file_name) = &context.file_name {
            cmds.push(text(left, header_y, file_name));
        }

        let channel_str = match context.audio.channels {
            1 => "Mono".to_string(),
            2 => "Stereo".to_string(),
            n => format!("{} ch", n),
        };

        let bit_depth_str = context
            .audio
            .bit_depth
            .map(|b| format!("{}-bit", b))
            .unwrap_or_else(|| "unknown bit".to_string());

        let audio_info = format!(
            "{} Hz · {} · {}",
            context.audio.sample_rate,
            channel_str,
            bit_depth_str
        );

        cmds.push(text(
            (left + right) / 2 - 80,
            header_y,
            &audio_info,
        ));

        if let Some(app_version) = &context.app_version {
            cmds.push(text(
                right.saturating_sub(140),
                header_y,
                app_version,
            ));
        }

        // -----------------------------------------------------------------
        // Axis lines
        // -----------------------------------------------------------------
        cmds.push(line(left, top, left, bottom));       // Frequency axis (left)
        cmds.push(line(left, bottom, right, bottom));   // Time axis
        cmds.push(line(right, top, right, bottom));     // dB axis / right frame

        // -----------------------------------------------------------------
        // Time axis ticks
        //  - bottom: ticks + labels
        //  - top: ticks only (no labels)
        // -----------------------------------------------------------------
        for i in 0..=settings.time_ticks {
            let t = i as f64 / settings.time_ticks as f64;
            let x = left + ((right - left) as f64 * t) as u32;

            // Bottom ticks
            cmds.push(line(x, bottom, x, bottom + 6));

            // Top ticks
            cmds.push(line(x, top.saturating_sub(6), x, top));

            // Bottom labels
            let total_seconds = context.duration_sec * t;
            let minutes = (total_seconds / 60.0).floor() as u64;
            let seconds = (total_seconds % 60.0).floor() as u64;

            cmds.push(text(
                x.saturating_sub(14),
                bottom + 10,
                &format!("{}:{:02}", minutes, seconds),
            ));
        }

        // X-axis title
        cmds.push(text(
            (left + right) / 2 - 18,
            bottom + 28,
            "Time",
        ));

        // -----------------------------------------------------------------
        // Frequency axis (linear, Hz → kHz)
        //  - labels LEFT
        //  - ticks LEFT + RIGHT (mirrored)
        // -----------------------------------------------------------------
        let nyquist = context.audio.sample_rate as f64 / 2.0;

        for i in 0..=settings.freq_ticks {
            let f = i as f64 / settings.freq_ticks as f64;
            let y = bottom - ((bottom - top) as f64 * f) as u32;
            let hz = nyquist * f;

            // Left ticks
            cmds.push(line(left - 6, y, left, y));

            // Right ticks (mirrored)
            cmds.push(line(right, y, right + 6, y));

            // Labels only on the left
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
                &format!("{:.0}", db),
            ));
        }

        // dB axis title
        cmds.push(text(
            right + 10,
            bottom + 28,
            "dBFS",
        ));

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
