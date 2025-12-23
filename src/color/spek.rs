//! Spek-style color mapper for spek-core.
//!
//! This is the canonical and ONLY color mapping used by spek-core.
//! It closely matches the original Spek / spek-rs visual appearance.
//!
//! - Deterministic
//! - Perceptually monotonic
//! - Matches Spek thermal palette (YUV-based)
//!
//! Input:  normalized intensity (0.0 .. 1.0)
//! Output: RGBA (8-bit per channel)

use crate::color::{ColorMapper, Rgba};

/// Canonical Spek color mapper.
///
/// Stateless and deterministic.
pub struct SpekColorMapper;

impl SpekColorMapper {
    pub fn new() -> Self {
        Self
    }
}

impl ColorMapper for SpekColorMapper {
    fn map(&self, intensity: f32) -> Rgba {
        let v = clamp01(intensity);

        // -----------------------------------------------------------------
        // Spek palette (approximated from spek-rs YUV gradient)
        //
        // intensity:
        //   0.0 -> dark violet / blue
        //   1.0 -> yellow / white
        // -----------------------------------------------------------------

        // Piecewise palette stops (v, y, u, v)
        let (y, u, v_) = if v < 0.25 {
            // dark → blue
            let t = v / 0.25;
            (
                lerp(0.05, 0.20, t),
                lerp(0.50, 0.60, t),
                lerp(0.60, 0.70, t),
            )
        } else if v < 0.5 {
            // blue → cyan
            let t = (v - 0.25) / 0.25;
            (
                lerp(0.20, 0.45, t),
                lerp(0.60, 0.45, t),
                lerp(0.70, 0.40, t),
            )
        } else if v < 0.75 {
            // cyan → yellow
            let t = (v - 0.5) / 0.25;
            (
                lerp(0.45, 0.75, t),
                lerp(0.45, 0.35, t),
                lerp(0.40, 0.25, t),
            )
        } else {
            // yellow → white
            let t = (v - 0.75) / 0.25;
            (
                lerp(0.75, 1.00, t),
                lerp(0.35, 0.50, t),
                lerp(0.25, 0.50, t),
            )
        };

        yuv_to_rgba(y, u, v_)
    }
}

// -------------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------------

#[inline]
fn clamp01(v: f32) -> f32 {
    if v < 0.0 {
        0.0
    } else if v > 1.0 {
        1.0
    } else {
        v
    }
}

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Convert normalized YUV (0..1) to RGBA (8-bit),
/// matching Spek / ffmpeg full-range behavior.
#[inline]
fn yuv_to_rgba(y: f32, u: f32, v: f32) -> Rgba {
    // Convert to 8-bit full-range YUV
    let y = y * 255.0;
    let u = 128.0 + (u - 0.5) * 255.0;
    let v = 128.0 + (v - 0.5) * 255.0;

    // Full-range YUV → RGB
    let r = y + 1.402 * (v - 128.0);
    let g = y - 0.344136 * (u - 128.0) - 0.714136 * (v - 128.0);
    let b = y + 1.772 * (u - 128.0);

    Rgba {
        r: r.clamp(0.0, 255.0) as u8,
        g: g.clamp(0.0, 255.0) as u8,
        b: b.clamp(0.0, 255.0) as u8,
        a: 255,
    }
}
