//! [sheety][crate] can be used to manipulate sprite sheets.
//! A sprite sheet can be created using [SpriteSheet].
//! [SpriteSheet::new] creates a new empty sprite sheet, where [SpriteSheet::load_div] and [SpriteSheet::load_cell_size]
//! are used to load a sheet from disk.
//!
//! # Example
//!
//! This is one of this crates examples - it concatenates to existing sprite sheets into a single.
//!
//! ```
//! SpriteSheet::concat(
//!     vec![
//!         SpriteSheet::load_cell_size("sheety/tests/machete_idle.png", (256, 256))
//!             .unwrap()
//!             .into_unordered()
//!             .unwrap(),
//!         SpriteSheet::load_cell_size("sheety/tests/machete_swing01.png", (256, 256))
//!             .unwrap()
//!             .into_unordered()
//!             .unwrap(),
//!     ]
//!     .into_iter(),
//!     Distribution::Packed(false),
//! )
//! .unwrap()
//! .save("sheety/tests/result_machete.png")
//! .unwrap();
//! ```

#![warn(missing_docs)]

extern crate image;

mod distribution;
pub mod error;
mod sprite;
mod sprite_cell;
mod sprite_sheet;
mod unordered_sprite_sheet;
mod utils;

pub use distribution::Distribution;
pub use sprite::Sprite;
pub use sprite_cell::SpriteCell;
pub use sprite_sheet::SpriteSheet;
pub use unordered_sprite_sheet::UnorderedSpriteSheet;
