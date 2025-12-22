//! Legend compositing for spek-core.
//!
//! Applies legend drawing commands onto an RGBA image buffer.
//! This module does NOT generate legend content.
//! It only executes drawing commands deterministically.

use crate::legend::LegendCommand;
use crate::render::ImageBuffer;

/// Composites legend commands onto an image buffer.
///
/// The legend is always applied AFTER spectrogram rendering.
pub struct LegendCompositor;

impl LegendCompositor {
    /// Apply legend drawing commands onto an image buffer.
    pub fn apply(
        image: &mut ImageBuffer,
        commands: &[LegendCommand],
        color: [u8; 4],
    ) {
        for cmd in commands {
            match cmd {
                LegendCommand::Line { x1, y1, x2, y2 } => {
                    Self::draw_line(image, *x1, *y1, *x2, *y2, color);
                }
                LegendCommand::Text { .. } => {
                    // Text rendering is delegated to a font rasterizer later.
                    // This compositor only reserves the command.
                }
            }
        }
    }

    fn draw_line(
        image: &mut ImageBuffer,
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
        color: [u8; 4],
    ) {
        let mut x = x1 as i32;
        let mut y = y1 as i32;
        let x2 = x2 as i32;
        let y2 = y2 as i32;

        let dx = (x2 - x).abs();
        let dy = -(y2 - y).abs();
        let sx = if x < x2 { 1 } else { -1 };
        let sy = if y < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            Self::set_pixel(image, x, y, color);

            if x == x2 && y == y2 {
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

    #[inline]
    fn set_pixel(image: &mut ImageBuffer, x: i32, y: i32, color: [u8; 4]) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= image.width || y >= image.height {
            return;
        }

        let idx = (y * image.width + x) * 4;
        image.data[idx] = color[0];
        image.data[idx + 1] = color[1];
        image.data[idx + 2] = color[2];
        image.data[idx + 3] = color[3];
    }
}
