use std::env;
use std::path::PathBuf;
use std::process::exit;

use spek_core::api::generate::generate_spectrogram;
use spek_core::api::settings::*;
use spek_core::audio::ffmpeg::FfmpegAudioSource;
use spek_core::analysis::fft::FftAnalyzer;
use spek_core::render::cpu::CpuRenderer;
use spek_core::legend::cpu::CpuLegendRenderer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.contains(&"--help".to_string()) {
        print_help();
        exit(0);
    }

    let input = PathBuf::from(&args[1]);
    let output = PathBuf::from(&args[2]);

    // ---- defaults -------------------------------------------------------
    let mut fft_size = 2048;
    let mut hop_size = 512;
    let mut width = 1024;
    let mut height = 512;
    let mut min_db = -120.0;
    let mut scale = ScaleMode::Log;

    // ---- parse flags ----------------------------------------------------
    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--fft" => fft_size = args[i + 1].parse().unwrap(),
            "--hop" => hop_size = args[i + 1].parse().unwrap(),
            "--width" => width = args[i + 1].parse().unwrap(),
            "--height" => height = args[i + 1].parse().unwrap(),
            "--min-db" => min_db = args[i + 1].parse().unwrap(),
            "--scale" => {
                scale = match args[i + 1].as_str() {
                    "linear" => ScaleMode::Linear,
                    "sqrt" => ScaleMode::Sqrt,
                    "cbrt" => ScaleMode::Cbrt,
                    "log" => ScaleMode::Log,
                    _ => panic!("Invalid scale mode"),
                }
            }
            _ => {}
        }
        i += 2;
    }

    // ---- construct settings --------------------------------------------
    let settings = SpekSettings {
        spectrogram: SpectrogramSettings {
            fft_size,
            hop_size,
            window: WindowFunction::Hann,
            channels: ChannelMode::Combined,
            min_db,
            max_db: 0.0,
            scale,
        },
        render: RenderSettings { width, height },
    };

    // ---- pipeline wiring ------------------------------------------------
    let source = FfmpegAudioSource::new(input);
    let analyzer = FftAnalyzer::new();
    let renderer = CpuRenderer::new();
    let legend = CpuLegendRenderer::new();

    let result = generate_spectrogram(
        &source,
        &analyzer,
        &renderer,
        &legend,
        &settings,
    )
    .expect("Spectrogram generation failed");

    // ---- write PNG ------------------------------------------------------
    std::fs::write(&output, result.image.data)
        .expect("Failed to write output image");
}

fn print_help() {
    println!(
        "Usage: spek-core <input_audio> <output_png> [options]

Options:
  --width <px>        Output width (default: 1024)
  --height <px>       Output height (default: 512)
  --fft <size>        FFT size (default: 2048)
  --hop <size>        Hop size (default: 512)
  --min-db <value>    Min dBFS (default: -120)
  --scale <mode>      linear|sqrt|cbrt|log
  --help"
    );
}
