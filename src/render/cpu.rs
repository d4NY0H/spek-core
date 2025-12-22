//! CPU-based spectrogram renderer for spek-core.
//!
//! Converts numerical spectrogram data into an RGBA pixel buffer.
//! This renderer does NOT draw legends, text, or axes.
//!
//! Deterministic, single-threaded, CPU-only.

use crate::analysis::{Spectrogram, SpectrogramSet};
use crate::render::{
    ChannelLayout, ImageBuffer, Orientation, RenderError, RenderSettings, Renderer,
};
use crate::color::{ColorMapper, Rgba};

/// Simple CPU renderer.
pub struct CpuRenderer;

impl CpuRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for CpuRenderer {
    fn render(
        &self,
        spectrograms: &SpectrogramSet,
        settings: &RenderSettings,
    ) -> Result<ImageBuffer, RenderError> {
        if settings.width == 0 || settings.height == 0 {
            return Err(RenderError::InvalidDimensions);
        }

        if spectrograms.channels.is_empty() {
            return Err(RenderError::Failed);
        }

        let channels = &spectrograms.channels;
        let channel_count = channels.len();

        let mut buffer =
            vec![0u8; settings.width * settings.height * 4];

        match settings.channels {
            ChannelLayout::Combined => {
                render_single(
                    &channels[0],
                    &mut buffer,
                    settings,
                );
            }
            ChannelLayout::Split => {
                let slice_height = settings.height / channel_count;
                for (i, spec) in channels.iter().enumerate() {
                    let y_offset = i * slice_height;
                    render_slice(
                        spec,
                        &mut buffer,
                        settings,
                        y_offset,
                        slice_height,
                    );
                }
            }
        }

        Ok(ImageBuffer {
            width: settings.width,
            height: settings.height,
            data: buffer,
        })
    }
}

/// Render a single spectrogram into the full image.
fn render_single(
    spec: &Spectrogram,
    buffer: &mut [u8],
    settings: &RenderSettings,
) {
    render_slice(
        spec,
        buffer,
        settings,
        0,
        settings.height,
    );
}

/// Render a spectrogram into a vertical slice.
fn render_slice(
    spec: &Spectrogram,
    buffer: &mut [u8],
    settings: &RenderSettings,
    y_offset: usize,
    slice_height: usize,
) {
    let time_bins = spec.time_bins;
    let freq_bins = spec.freq_bins;

    for y in 0..slice_height {
        let freq_idx = match settings.orientation {
            Orientation::Vertical => {
                ((slice_height - 1 - y) * freq_bins) / slice_height
            }
            Orientation::Horizontal => {
                (y * freq_bins) / slice_height
            }
        };

        if freq_idx >= freq_bins {
            continue;
        }

        for x in 0..settings.width {
            let time_idx = (x * time_bins) / settings.width;
            if time_idx >= time_bins {
                continue;
            }

            let intensity = spec.data[freq_idx][time_idx];
            let gray = (intensity * 255.0).clamp(0.0, 255.0) as u8;

            write_pixel(
                buffer,
                settings.width,
                x,
                y + y_offset,
                Rgba {
                    r: gray,
                    g: gray,
                    b: gray,
                    a: 255,
                },
            );
        }
    }
}

/// Write a single RGBA pixel.
#[inline]
fn write_pixel(
    buffer: &mut [u8],
    width: usize,
    x: usize,
    y: usize,
    color: Rgba,
) {
    let idx = (y * width + x) * 4;
    buffer[idx] = color.r;
    buffer[idx + 1] = color.g;
    buffer[idx + 2] = color.b;
    buffer[idx + 3] = color.a;
}
