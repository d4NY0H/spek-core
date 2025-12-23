//! Legend pixel overlay for spek-core.
//!
//! Applies legend drawing commands onto an existing RGBA image buffer.
//! This module does NOT generate legend content.
//! It only executes drawing commands.

use crate::legend::LegendCommand;
use crate::render::ImageBuffer;

/// Apply legend commands onto an RGBA image buffer.
///
/// The image buffer is modified in-place.
pub fn apply_legend_overlay(
    image: &mut ImageBuffer,
    commands: &[LegendCommand],
) {
    for cmd in commands {
        match cmd {
            LegendCommand::Text { x, y, content } => {
                draw_text_stub(image, *x, *y, content);
            }

            LegendCommand::Line { x1, y1, x2, y2 } => {
                // Plain white line (axes, ticks, frames)
                draw_line(image, *x1, *y1, *x2, *y2);
            }

            LegendCommand::DbfsGradient { x, y_top, y_bottom } => {
                draw_dbfs_gradient(image, *x, *y_top, *y_bottom);
            }
        }
    }
}

/// Draw a vertical dBFS gradient bar (Spek-style).
///
/// Semantic guarantees:
/// - Top = 0 dBFS (bright)
/// - Bottom = min dBFS (dark)
/// - Does NOT touch any other pixels
fn draw_dbfs_gradient(
    image: &mut ImageBuffer,
    x: u32,
    y_top: u32,
    y_bottom: u32,
) {
    let (start, end) = if y_top <= y_bottom {
        (y_top, y_bottom)
    } else {
        (y_bottom, y_top)
    };

    let height = (end - start).max(1) as f32;

    for y in start..=end {
        let t = (y - start) as f32 / height;
        let a = 1.0 - t; // top = bright

        // Spek-like pseudo-thermal palette
        let (r, g, b) = spek_palette(a);

        put_pixel(image, x, y, r, g, b, 255);
    }
}

/// Spek-style palette approximation (matches visual behavior)
#[inline]
fn spek_palette(a: f32) -> (u8, u8, u8) {
    let a = a.clamp(0.0, 1.0);

    if a < 0.25 {
        // black → blue
        let t = a / 0.25;
        (0, 0, (255.0 * t) as u8)
    } else if a < 0.5 {
        // blue → cyan
        let t = (a - 0.25) / 0.25;
        (0, (255.0 * t) as u8, 255)
    } else if a < 0.75 {
        // cyan → yellow
        let t = (a - 0.5) / 0.25;
        ((255.0 * t) as u8, 255, (255.0 * (1.0 - t)) as u8)
    } else {
        // yellow → white
        let t = (a - 0.75) / 0.25;
        (255, 255, (255.0 * t) as u8)
    }
}

/// Draw a line using simple Bresenham algorithm.
fn draw_line(
    image: &mut ImageBuffer,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
) {
    let mut x = x1 as i32;
    let mut y = y1 as i32;

    let dx = (x2 as i32 - x1 as i32).abs();
    let dy = -(y2 as i32 - y1 as i32).abs();

    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };

    let mut err = dx + dy;

    loop {
        put_pixel(image, x as u32, y as u32, 255, 255, 255, 255);

        if x == x2 as i32 && y == y2 as i32 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

/// Placeholder text renderer.
///
/// Real font rasterization will be injected later.
/// This keeps spek-core font-backend-agnostic.
fn draw_text_stub(
    _image: &mut ImageBuffer,
    _x: u32,
    _y: u32,
    _text: &str,
) {
    // intentionally empty
}

/// Write a single RGBA pixel.
#[inline]
fn put_pixel(
    image: &mut ImageBuffer,
    x: u32,
    y: u32,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    if x >= image.width as u32 || y >= image.height as u32 {
        return;
    }

    let idx = (y as usize * image.width + x as usize) * 4;
    if idx + 3 < image.data.len() {
        image.data[idx] = r;
        image.data[idx + 1] = g;
        image.data[idx + 2] = b;
        image.data[idx + 3] = a;
    }
}
