//! # This prison... To hold... Me ?

// #![warn(missing_docs)]

extern crate image;

pub mod error;
mod sprite;
mod sprite_cell;
mod sprite_sheet;
mod unordered_sprite_sheet;
mod utils;

pub use sprite::Sprite;
pub use sprite_cell::SpriteCell;
pub use sprite_sheet::SpriteSheet;
pub use unordered_sprite_sheet::UnorderedSpriteSheet;
pub use utils::IVec2;
