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
                draw_line(image, *x1, *y1, *x2, *y2);
            }
        }
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
    // Intentionally empty.
    // Font rasterization is a replaceable backend.
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

    let idx = ((y as usize * image.width + x as usize) * 4) as usize;
    if idx + 3 < image.data.len() {
        image.data[idx] = r;
        image.data[idx + 1] = g;
        image.data[idx + 2] = b;
        image.data[idx + 3] = a;
    }
}
