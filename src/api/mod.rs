//! spek-core public API
//!
//! This module defines the ONLY supported entry points into the core.
//! No UI, no platform logic, no side effects.

// ---------------------------------------------------------------------
// Submodules
// ---------------------------------------------------------------------

pub mod generate;
pub mod settings;
pub mod result;

// ---------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------

use crate::audio::AudioSource;

use crate::analysis::fft::FftAnalyzer;
use crate::color::spek::SpekColorMapper;
use crate::render::basic::BasicRenderer;
use crate::legend::simple::SimpleLegendRenderer;

use generate::GenerateError;
use settings::{SpectrogramSettings, SpekSettings};
use result::SpectrogramResult;

/// Errors returned by spek-core.
///
/// This type is intentionally small and stable.
#[derive(Debug)]
pub enum SpekError {
    InvalidInput,
    DecodeError,
    AnalysisError,
    RenderError,
    Cancelled,
}

/// Generate a spectrogram image with a mandatory legend.
///
/// This function is:
/// - deterministic
/// - blocking
/// - side-effect free
///
/// One call = one image.
///
/// This function wires the DEFAULT core components.
pub fn generate_spectrogram(
    source: &dyn AudioSource,
    settings: &SpectrogramSettings,
) -> Result<SpectrogramResult, SpekError> {
    // -----------------------------------------------------------------
    // Instantiate default core components
    // -----------------------------------------------------------------
    let analyzer = FftAnalyzer::new();
    let color_mapper = SpekColorMapper::new();
    let renderer = BasicRenderer::new(&color_mapper);
    let legend = SimpleLegendRenderer::new();

    let spek_settings = SpekSettings {
        spectrogram: settings.clone(),
        render: settings.render.clone(),
    };

    generate::generate_spectrogram(
        source,
        &analyzer,
        &renderer,
        &color_mapper,
        &legend,
        &spek_settings,
    )
    .map_err(|e| match e {
        GenerateError::DecodeFailed => SpekError::DecodeError,
        GenerateError::AnalysisFailed => SpekError::AnalysisError,
        GenerateError::RenderFailed => SpekError::RenderError,
    })
}
