//! Basic spectrogram renderer for spek-core.
//!
//! Converts numerical spectrogram data into an RGBA image buffer.
//! This renderer has NO knowledge of legends, fonts, or text.
//! Color mapping is owned by the renderer instance.

use crate::analysis::SpectrogramSet;
use crate::color::ColorMapper;
use crate::render::{
    ChannelLayout, ImageBuffer, Orientation, RenderError, RenderSettings, Renderer,
};

/// Basic deterministic spectrogram renderer.
pub struct BasicRenderer<'a> {
    pub color_mapper: &'a dyn ColorMapper,
}

impl<'a> BasicRenderer<'a> {
    pub fn new(color_mapper: &'a dyn ColorMapper) -> Self {
        Self { color_mapper }
    }
}

impl<'a> Renderer for BasicRenderer<'a> {
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

        let channel_count = spectrograms.channels.len();
        let spec = &spectrograms.channels[0];
        let freq_bins = spec.freq_bins;
        let time_bins = spec.time_bins;

        let mut buffer = vec![0u8; settings.width * settings.height * 4];

        for y in 0..settings.height {
            for x in 0..settings.width {
                let (time_idx, freq_idx, channel_idx) = match settings.channels {
                    ChannelLayout::Combined => {
                        let (t, f) = match settings.orientation {
                            Orientation::Vertical => {
                                let t = x * time_bins / settings.width;
                                let f =
                                    (settings.height - 1 - y) * freq_bins / settings.height;
                                (t, f)
                            }
                            Orientation::Horizontal => {
                                let t = y * time_bins / settings.height;
                                let f =
                                    (settings.width - 1 - x) * freq_bins / settings.width;
                                (t, f)
                            }
                        };

                        (
                            t.min(time_bins.saturating_sub(1)),
                            f.min(freq_bins.saturating_sub(1)),
                            None,
                        )
                    }

                    ChannelLayout::Split => {
                        if settings.height < channel_count {
                            return Err(RenderError::InvalidDimensions);
                        }

                        let ch_height = settings.height / channel_count;
                        let ch = (y / ch_height).min(channel_count - 1);
                        let local_y = y % ch_height;

                        let t = x * time_bins / settings.width;
                        let f = (ch_height - 1 - local_y) * freq_bins / ch_height;

                        (
                            t.min(time_bins.saturating_sub(1)),
                            f.min(freq_bins.saturating_sub(1)),
                            Some(ch),
                        )
                    }
                };

                let intensity = match channel_idx {
                    None => {
                        let mut sum = 0.0;
                        for ch in &spectrograms.channels {
                            sum += ch.data[freq_idx][time_idx];
                        }
                        sum / spectrograms.channels.len() as f32
                    }
                    Some(ch) => spectrograms.channels[ch].data[freq_idx][time_idx],
                };

                let color = self.color_mapper.map(intensity);
                let idx = (y * settings.width + x) * 4;

                buffer[idx] = color.r;
                buffer[idx + 1] = color.g;
                buffer[idx + 2] = color.b;
                buffer[idx + 3] = color.a;
            }
        }

        Ok(ImageBuffer {
            width: settings.width,
            height: settings.height,
            data: buffer,
        })
    }
}
