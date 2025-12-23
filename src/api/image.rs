//! Image types for spek-core.
//!
//! This module defines the raw RGBA image buffer used throughout the API.
//! It is backend-agnostic and renderer-independent.

/// RGBA8 image buffer (row-major).
///
/// Layout:
/// data.len() == width * height * 4
#[derive(Debug, Clone)]
pub struct ImageBuffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}
