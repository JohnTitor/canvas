// Distributed under The MIT License (MIT)
//
// Copyright (c) 2019 The `image-rs` developers
//! # Canvas
//!
//! An image canvas compatible with transmuting its byte content.
//!
//! ## Usage
//!
//! ```
//! # fn send_over_network(_: &[u8]) { };
//! use canvas::Canvas;
//! let mut canvas = Canvas::with_width_and_height(400, 400);
//!
//! // Draw a bright red line.
//! for i in 0..400 {
//!     // Assign color as u8-RGBA
//!     canvas[(i, i)] = [0xFF, 0x00, 0x00, 0xFF];
//! }
//!
//! // Encode to network endian.
//! let mut encoded = canvas.transmute::<u32>();
//! encoded
//!     .as_mut_slice()
//!     .iter_mut()
//!     .for_each(|p| *p = p.to_be());
//!
//! // Send the raw bytes
//! send_over_network(encoded.as_bytes());
//! ```
mod buf;
mod canvas;
mod pixel;
mod rec;

pub use self::canvas::{Canvas, CanvasReuseError, Layout};
pub use self::pixel::{AsPixel, Pixel};
pub use self::rec::{Rec, ReuseError};

/// Constants for predefined pixel types.
pub mod pixels {
    pub use crate::pixel::constants::*;
    pub use crate::pixel::MaxAligned;
}
