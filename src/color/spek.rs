//! Spek-style color mapper for spek-core.
//!
//! This is the canonical and ONLY color mapping used by spek-core.
//! It is designed to closely match the original Spek visual appearance.
//!
//! - Deterministic
//! - Perceptually monotonic
//! - High contrast for low-energy signals
//!
//! Input:  normalized intensity (0.0 .. 1.0)
//! Output: RGBA (8-bit per channel)

use crate::color::{ColorMapper, Rgba};

/// Canonical Spek color mapper.
///
/// This mapper is stateless and deterministic.
pub struct SpekColorMapper;

impl SpekColorMapper {
    pub fn new() -> Self {
        Self
    }
}

impl ColorMapper for SpekColorMapper {
    fn map(&self, intensity: f32) -> Rgba {
        let v = clamp01(intensity);

        // Spek-style pseudo-thermal gradient
        // black → blue → cyan → yellow → white

        let (r, g, b) = if v < 0.25 {
            // black → blue
            let t = v / 0.25;
            (0.0, 0.0, 255.0 * t)
        } else if v < 0.5 {
            // blue → cyan
            let t = (v - 0.25) / 0.25;
            (0.0, 255.0 * t, 255.0)
        } else if v < 0.75 {
            // cyan → yellow
            let t = (v - 0.5) / 0.25;
            (255.0 * t, 255.0, 255.0 * (1.0 - t))
        } else {
            // yellow → white
            let t = (v - 0.75) / 0.25;
            (255.0, 255.0, 255.0 * t)
        };

        Rgba {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a: 255,
        }
    }
}

/// Clamp helper.
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
