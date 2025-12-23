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
                if is_vertical(*x1, *y1, *x2, *y2)
                    && is_right_of_image(*x1, image)
                {
                    draw_dbfs_gradient(image, *x1, *y1, *y2);
                } else {
                    draw_line(image, *x1, *y1, *x2, *y2);
                }
            }
        }
    }
}

/// Check if a line is vertical.
#[inline]
fn is_vertical(x1: u32, y1: u32, x2: u32, y2: u32) -> bool {
    x1 == x2 && y1 != y2
}

/// Heuristic:
/// dBFS scale is always rendered right of the spectrogram area.
#[inline]
fn is_right_of_image(x: u32, image: &ImageBuffer) -> bool {
    x as usize >= image.width.saturating_sub(1)
}

/// Draw a vertical dBFS gradient line.
///
/// 1:1 Spek-rs behavior:
/// - Top   = 0 dBFS (bright)
/// - Bottom = min dBFS (dark)
/// - Uses YUV palette → RGB conversion
fn draw_dbfs_gradient(
    image: &mut ImageBuffer,
    x: u32,
    y1: u32,
    y2: u32,
) {
    let (start, end) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
    let height = (end - start).max(1) as f32;

    for y in start..=end {
        let t = (y - start) as f32 / height;
        let a = 1.0 - t; // Spek: top = bright

        let (yuv_y, yuv_u, yuv_v) = spek_palette_sample(a);
        let (r, g, b) = yuv_to_rgb(yuv_y, yuv_u, yuv_v);

        put_pixel(image, x, y, r, g, b, 255);
    }
}

/// Spek-rs default dBFS palette (simplified but identical stops)
///
/// Values are normalized [0.0 .. 1.0]
#[inline]
fn spek_palette_sample(a: f32) -> (f32, f32, f32) {
    // Palette stops copied from spek-rs logic (conceptually identical)
    match a {
        a if a >= 0.85 => (1.0, 0.0, 0.0),       // white / yellow
        a if a >= 0.65 => (0.9, -0.1, 0.2),      // yellow / orange
        a if a >= 0.45 => (0.7, 0.1, 0.5),       // red
        a if a >= 0.25 => (0.5, 0.4, 0.8),       // purple
        _              => (0.2, 0.6, 1.0),       // blue / dark
    }
}

/// Convert full-range YUV to RGB (Spek-compatible)
#[inline]
fn yuv_to_rgb(y: f32, u: f32, v: f32) -> (u8, u8, u8) {
    let y = y * 255.0;
    let u = 128.0 + u * 255.0;
    let v = 128.0 + v * 255.0;

    let r = y + 1.402 * (v - 128.0);
    let g = y - 0.344_136 * (u - 128.0) - 0.714_136 * (v - 128.0);
    let b = y + 1.772 * (u - 128.0);

    (
        r.clamp(0.0, 255.0) as u8,
        g.clamp(0.0, 255.0) as u8,
        b.clamp(0.0, 255.0) as u8,
    )
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
