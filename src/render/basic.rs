//! Basic spectrogram renderer for spek-core.
//!
//! Converts numerical spectrogram data into an RGBA image buffer.
//! This renderer has NO knowledge of legends, fonts, or text.
//! Color mapping is injected via a ColorMapper.

use crate::analysis::SpectrogramSet;
use crate::color::{ColorMapper, Rgba};
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

        let channels = match settings.channels {
            ChannelLayout::Combined => 1,
            ChannelLayout::Split => spectrograms.channels.len(),
        };

        let spec = &spectrograms.channels[0];
        let freq_bins = spec.freq_bins;
        let time_bins = spec.time_bins;

        let mut buffer = vec![0u8; settings.width * settings.height * 4];

        for y in 0..settings.height {
            for x in 0..settings.width {
                let (time_idx, freq_idx) = match settings.orientation {
                    Orientation::Vertical => {
                        let t = x * time_bins / settings.width;
                        let f = (settings.height - 1 - y) * freq_bins / settings.height;
                        (t, f)
                    }
                    Orientation::Horizontal => {
                        let t = y * time_bins / settings.height;
                        let f = (settings.width - 1 - x) * freq_bins / settings.width;
                        (t, f)
                    }
                };

                let mut intensity = 0.0;

                match settings.channels {
                    ChannelLayout::Combined => {
                        for ch in &spectrograms.channels {
                            intensity += ch.data[freq_idx][time_idx];
                        }
                        intensity /= spectrograms.channels.len() as f32;
                    }
                    ChannelLayout::Split => {
                        let ch_height = settings.height / channels;
                        let ch_index = y / ch_height;
                        let ch_index = ch_index.min(spectrograms.channels.len() - 1);
                        intensity = spectrograms.channels[ch_index].data[freq_idx][time_idx];
                    }
                }

                let Rgba { r, g, b, a } = self.color_mapper.map(intensity);
                let idx = (y * settings.width + x) * 4;

                buffer[idx] = r;
                buffer[idx + 1] = g;
                buffer[idx + 2] = b;
                buffer[idx + 3] = a;
            }
        }

        Ok(ImageBuffer {
            width: settings.width,
            height: settings.height,
            data: buffer,
        })
    }
}
