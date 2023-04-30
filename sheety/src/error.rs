//! Contains all error-related types.

use image::ImageError;

use thiserror::Error;

use crate::utils::IVec2;

/// An error returned by a [sheety][crate] function.
#[derive(Debug, Error)]
pub enum Error {
    /// The specified coordonates are out of bounds.
    #[error("the specified coordonates `{provided:?}` are out of bounds (max: `{max:?}`)")]
    OutOfBounds {
        /// The maximum coordonates of the current [Sprite][crate::Sprite]/[SpriteSheet][crate::SpriteSheet]
        max: IVec2,
        /// The provided coordonates, which are out of bounds.
        provided: IVec2,
    },
    /// Attempted to make a [SpriteSheet][crate::SpriteSheet] or an [UnorderedSpriteSheet][crate::UnorderedSpriteSheet]
    /// from [Sprite][crate::Sprite]s of different sizes.
    #[error("attempted to make a sprite sheet or an unordered sprite sheet from sprites of different sizes \
	(started with size `{required:?}`, encountered size `{provided:?}`)")]
    MismatchedSpriteSize {
        /// The determined size, gotten from the first elements of the sheet.
        required: IVec2,
        /// The first size found to be different from the required size.
        provided: IVec2,
    },
    /// Attempted to construct an [UnorderedSpriteSheet][crate::UnorderedSpriteSheet] with no [Sprite][crate::Sprite]s.
    /// At least one [Sprite][crate::Sprite] is needed to guess the cell size of the sheet.
    #[error("attempted to make an empty unordered sprite sheet")]
    EmptyUnorderedSpriteSheet,
    /// Attempted to concatenate [UnorderedSpriteSheet][crate::UnorderedSpriteSheet]s from a empty iterator.
    #[error("attempted to concatenate unordered sprite sheets from an empty iterator")]
    EmptyIterator,
    /// Attempted to push one or more [Sprite][crate::Sprite]s into a full [SpriteSheet][crate::SpriteSheet].
    #[error("attempted to push one or more sprites into a full sprite sheet (sprites that were able to be fitted: `{amount_fitted}`)")]
    SheetFull {
        /// The amount of [Sprite][crate::Sprite]s that were able to be fitted.
        amount_fitted: u32,
    },
    /// Encapsulates an [image::ImageError] from the [image] crate.
    #[error("internal image error")]
    ImageError(#[from] ImageError),
}

/// Type alias for `Result<T, sheety::Error>`
pub type Result<T> = std::result::Result<T, Error>;
