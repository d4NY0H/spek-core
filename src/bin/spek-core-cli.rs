//! spek-core CLI wrapper (Linux / headless).
//!
//! NOTE:
//! This is a STUB CLI.
//! Audio decoding is intentionally mocked so the core can build & link.
//!
//! Usage:
//!   spek-core-cli <ignored_input> <output.png>

use std::env;
use std::path::PathBuf;

use spek_core::api::generate::generate_spectrogram;
use spek_core::api::settings::{
    ChannelMode, RenderSettings, ScaleMode, SpekSettings, SpectrogramSettings, WindowFunction,
};
use spek_core::analysis::fft::FftAnalyzer;
use spek_core::audio::{AudioBuffer, AudioMetadata, AudioSource};
use spek_core::color::spek::SpekColorMapper;
use spek_core::render::basic::BasicRenderer;
use spek_core::legend::simple::SimpleLegendRenderer;

/// ---------------------------------------------------------------------
/// Dummy audio source (STUB)
/// ---------------------------------------------------------------------
/// Produces a short silent buffer so the full pipeline can run.
struct DummyAudioSource;

impl AudioSource for DummyAudioSource {
    fn load(&self) -> Result<AudioBuffer, spek_core::audio::AudioError> {
        let sample_rate = 44100;
        let seconds = 2;
        let samples = vec![0.0f32; sample_rate * seconds];

        Ok(AudioBuffer {
            samples,
            meta: AudioMetadata {
                sample_rate: sample_rate as u32,
                channels: 1,
                total_samples: samples.len() as u64,
                bit_depth: None,
            },
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: spek-core-cli <ignored_input> <output.png>");
        std::process::exit(1);
    }

    let output = PathBuf::from(&args[2]);

    // -----------------------------------------------------------------
    // Instantiate core components
    // -----------------------------------------------------------------
    let audio = DummyAudioSource;
    let analyzer = FftAnalyzer::new();
    let color_mapper = SpekColorMapper::new();
    let renderer = BasicRenderer::new(&color_mapper);
    let legend = SimpleLegendRenderer::new();

    // -----------------------------------------------------------------
    // Default deterministic Spek-style settings
    // -----------------------------------------------------------------
    let settings = SpekSettings {
        spectrogram: SpectrogramSettings {
            fft_size: 2048,
            hop_size: 512,
            window: WindowFunction::Hann,
            channels: ChannelMode::Combined,
            min_db: -120.0,
            max_db: 0.0,
            scale: ScaleMode::Log,
        },
        render: RenderSettings {
            width: 1024,
            height: 512,
        },
    };

    // -----------------------------------------------------------------
    // Run pipeline
    // -----------------------------------------------------------------
    let result = match generate_spectrogram(
        &audio,
        &analyzer,
        &renderer,
        &legend,
        &settings,
    ) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            std::process::exit(2);
        }
    };

    // -----------------------------------------------------------------
    // Write PNG
    // -----------------------------------------------------------------
    if let Err(e) = write_png(&output, &result.image) {
        eprintln!("Failed to write PNG: {}", e);
        std::process::exit(3);
    }
}

/// Write RGBA buffer to PNG using image crate.
fn write_png(
    path: &PathBuf,
    image: &spek_core::render::ImageBuffer,
) -> Result<(), String> {
    let img = image::RgbaImage::from_raw(
        image.width as u32,
        image.height as u32,
        image.data.clone(),
    )
    .ok_or("Invalid image buffer")?;

    img.save(path).map_err(|e| e.to_string())
}
