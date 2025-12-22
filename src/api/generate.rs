//! Public spectrogram generation API.

use crate::api::result::SpectrogramResult;
use crate::api::settings::SpectrogramSettings;

/// Generate a spectrogram image including legend.
///
/// This is the single public entry point of spek-core.
///
/// One call → one deterministic result.
///
/// # Errors
///
/// Returns an error if:
/// - the input audio cannot be read
/// - decoding fails
/// - processing is aborted
pub fn generate_spectrogram(
    input_path: &str,
    settings: &SpectrogramSettings,
) -> Result<SpectrogramResult, String> {
    // Implementation will be wired here step by step:
    // audio → analysis → render → legend → result

    Err("Not implemented yet".into())
}
