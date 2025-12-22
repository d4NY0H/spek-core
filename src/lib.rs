//! spek-core
//!
//! Platform-independent, headless spectrogram core.
//! No UI, no windowing, no platform assumptions.

pub mod api;

pub mod audio;
pub mod analysis;
pub mod render;
pub mod legend;
pub mod color;

// Re-export public API
pub use api::*;
