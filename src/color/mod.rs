//! Color mapping for spek-core.
//!
//! Maps normalized intensity values (0.0–1.0) to RGBA colors.
//! This module has no knowledge of time, frequency, or legends.

/// RGBA color (8-bit per channel).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Supported color palettes.
#[derive(Debug, Copy, Clone)]
pub enum Palette {
    Grayscale,
    Viridis,
    Plasma,
    Magma,
    Inferno,
}

/// Color mapping settings.
#[derive(Debug, Clone)]
pub struct ColorSettings {
    /// Selected color palette
    pub palette: Palette,

    /// Global saturation multiplier
    pub saturation: f32,

    /// Optional inversion
    pub invert: bool,
}

/// Color mapper interface.
pub trait ColorMapper {
    /// Map normalized intensity to RGBA.
    ///
    /// Input is expected to be in range 0.0–1.0.
    fn map(&self, intensity: f32) -> Rgba;
}

/// Clamp helper.
#[inline]
pub fn clamp01(v: f32) -> f32 {
    if v < 0.0 {
        0.0
    } else if v > 1.0 {
        1.0
    } else {
        v
    }
}
