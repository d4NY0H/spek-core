//! CPU-based color mapping for spek-core.
//!
//! Maps normalized intensity values (0.0–1.0) to RGBA colors.
//! This module has NO knowledge of time, frequency, or legends.

use crate::color::{clamp01, ColorMapper, ColorSettings, Palette, Rgba};

/// CPU color mapper.
#[derive(Debug, Clone)]
pub struct CpuColorMapper {
    settings: ColorSettings,
}

impl CpuColorMapper {
    pub fn new(settings: ColorSettings) -> Self {
        Self { settings }
    }

    fn map_palette(&self, v: f32) -> Rgba {
        match self.settings.palette {
            Palette::Grayscale => grayscale(v),
            Palette::Viridis => viridis(v),
            Palette::Plasma => plasma(v),
            Palette::Magma => magma(v),
            Palette::Inferno => inferno(v),
        }
    }
}

impl ColorMapper for CpuColorMapper {
    fn map(&self, intensity: f32) -> Rgba {
        let mut v = clamp01(intensity);

        if self.settings.invert {
            v = 1.0 - v;
        }

        let mut color = self.map_palette(v);

        if self.settings.saturation != 1.0 {
            color = apply_saturation(color, self.settings.saturation);
        }

        color
    }
}

/// Apply saturation multiplier in RGB space.
fn apply_saturation(c: Rgba, s: f32) -> Rgba {
    let gray = (c.r as f32 + c.g as f32 + c.b as f32) / 3.0;

    let mix = |v: u8| -> u8 {
        let vf = v as f32;
        ((gray + (vf - gray) * s).clamp(0.0, 255.0)) as u8
    };

    Rgba {
        r: mix(c.r),
        g: mix(c.g),
        b: mix(c.b),
        a: c.a,
    }
}

/// Grayscale palette.
fn grayscale(v: f32) -> Rgba {
    let g = (v * 255.0) as u8;
    Rgba {
        r: g,
        g: g,
        b: g,
        a: 255,
    }
}

/// Viridis approximation.
fn viridis(v: f32) -> Rgba {
    let x = clamp01(v);
    Rgba {
        r: (255.0 * (0.267 + x * 0.633)) as u8,
        g: (255.0 * (0.005 + x * 0.995)) as u8,
        b: (255.0 * (0.329 + x * 0.336)) as u8,
        a: 255,
    }
}

/// Plasma approximation.
fn plasma(v: f32) -> Rgba {
    let x = clamp01(v);
    Rgba {
        r: (255.0 * (0.941 + x * 0.059)) as u8,
        g: (255.0 * (0.173 + x * 0.827)) as u8,
        b: (255.0 * (0.074 + x * 0.926)) as u8,
        a: 255,
    }
}

/// Magma approximation.
fn magma(v: f32) -> Rgba {
    let x = clamp01(v);
    Rgba {
        r: (255.0 * (0.987 * x)) as u8,
        g: (255.0 * (0.291 * x)) as u8,
        b: (255.0 * (0.232 * x)) as u8,
        a: 255,
    }
}

/// Inferno approximation.
fn inferno(v: f32) -> Rgba {
    let x = clamp01(v);
    Rgba {
        r: (255.0 * (0.995 * x)) as u8,
        g: (255.0 * (0.447 * x)) as u8,
        b: (255.0 * (0.122 * x)) as u8,
        a: 255,
    }
}
