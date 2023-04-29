use image::ImageError;

use crate::utils::IVec2;

#[derive(Debug)]
pub enum Error {
    OutOfRange { max: IVec2, provided: IVec2 },
    MismatchedSpriteSize { required: IVec2, provided: IVec2 },
    EmptyUnorderedSpriteSheet,
    EmptyIterator,
    SheetFull { amount_fitted: u32 },
    ImageError(ImageError),
}

pub type Result<T> = std::result::Result<T, Error>;
