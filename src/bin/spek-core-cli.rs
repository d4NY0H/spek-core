//! spek-core CLI wrapper (Linux / headless).
//!
//! Usage:
//!   spek-core-cli <input_audio> <output.png>

use std::env;
use std::path::PathBuf;

use spek_core::api::generate::generate_spectrogram;
use spek_core::api::settings::{
    ChannelMode, RenderSettings, ScaleMode, SpekSettings, SpectrogramSettings, WindowFunction,
};
use spek_core::audio::ffmpeg::FfmpegAudioSource;
use spek_core::analysis::fft::FftAnalyzer;
use spek_core::color::spek::SpekColorMapper;
use spek_core::render::basic::BasicRenderer;
use spek_core::legend::simple::SimpleLegendRenderer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: spek-core-cli <input_audio> <output.png>");
        std::process::exit(1);
    }

    let input = PathBuf::from(&args[1]);
    let output = PathBuf::from(&args[2]);

    // ---------------------------------------------------------------------
    // Instantiate core components
    // ---------------------------------------------------------------------
    let audio = FfmpegAudioSource::new(input);
    let analyzer = FftAnalyzer::new();
    let color_mapper = SpekColorMapper::new();
    let renderer = BasicRenderer::new(&color_mapper);
    let legend = SimpleLegendRenderer::new();

    // ---------------------------------------------------------------------
    // Default settings (deterministic, Spek-like)
    // ---------------------------------------------------------------------
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

    // ---------------------------------------------------------------------
    // Run pipeline
    // ---------------------------------------------------------------------
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

    // ---------------------------------------------------------------------
    // Write PNG
    // ---------------------------------------------------------------------
    if let Err(e) = write_png(&output, &result.image) {
        eprintln!("Failed to write PNG: {}", e);
        std::process::exit(3);
    }
}

/// Write RGBA buffer to PNG using image crate.
fn write_png(
    path: &PathBuf,
    image: &spek_core::api::result::ImageBuffer,
) -> Result<(), String> {
    let img = image::RgbaImage::from_raw(
        image.width,
        image.height,
        image.data.clone(),
    )
    .ok_or("Invalid image buffer")?;

    img.save(path).map_err(|e| e.to_string())
}
