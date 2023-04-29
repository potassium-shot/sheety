//! Contains all error-related types.

use image::ImageError;

use crate::utils::IVec2;

/// An error returned by a [sheety][crate] function.
#[derive(Debug)]
pub enum Error {
    /// The specified coordonates are out of bounds.
    OutOfBounds {
        /// The maximum coordonates of the current [Sprite][crate::Sprite]/[SpriteSheet][crate::SpriteSheet]
        max: IVec2,
        /// The provided coordonates, which are out of bounds.
        provided: IVec2,
    },
    /// Attempted to make a [SpriteSheet][crate::SpriteSheet] or an [UnorderedSpriteSheet][crate::UnorderedSpriteSheet]
    /// from [Sprite][crate::Sprite]s of different sizes.
    MismatchedSpriteSize {
        /// The determined size, gotten from the first elements of the sheet.
        required: IVec2,
        /// The first size found to be different from the required size.
        provided: IVec2,
    },
    /// Attempted to construct an [UnorderedSpriteSheet][crate::UnorderedSpriteSheet] with no [Sprite][crate::Sprite]s.
    /// At least one [Sprite][crate::Sprite] is needed to guess the cell size of the sheet.
    EmptyUnorderedSpriteSheet,
    /// Attempted to concatenate [UnorderedSpriteSheet][crate::UnorderedSpriteSheet]s from a empty iterator.
    EmptyIterator,
    /// Attempted to push one or more [Sprite][crate::Sprite]s into a full [SpriteSheet][crate::SpriteSheet].
    SheetFull {
        /// The amount of [Sprite][crate::Sprite]s that were able to be fitted.
        amount_fitted: u32,
    },
    /// Encapsulates an [image::ImageError] from the [image] crate.
    ImageError(ImageError),
}

/// Type alias for `Result<T, sheety::Error>`
pub type Result<T> = std::result::Result<T, Error>;
