use std::path::Path;

use image::{DynamicImage, RgbaImage};

use crate::{
    error::{Error, Result},
    utils::IVec2,
};

/// Wrapper around [image::RgbaImage] that allows use with [SpriteSheet][crate::SpriteSheet]
/// and [UnorederedSpriteSheet][crate::UnorderedSpriteSheet]
#[derive(Debug, Clone)]
pub struct Sprite {
    image: RgbaImage,
}

impl Sprite {
    /// Attempts to load a [Sprite] by openning an image.
    ///
    /// # Errors
    ///
    /// - Will return [Error::ImageError] if the underlying call to [image::open] returns an error.
    pub fn load<P>(path: P) -> Result<Sprite>
    where
        P: AsRef<Path>,
    {
        Ok(Self::from(image::open(path).map_err(Error::ImageError)?))
    }

    /// Gets the size of the underlying [image::RgbaImage].
    pub fn size(&self) -> IVec2 {
        (self.image.width() as usize, self.image.height() as usize)
    }

    /// Consumes this [Sprite] and returns its underlying [image::RgbaImage].
    pub fn into_image(self) -> RgbaImage {
        self.image
    }

    /// Returns `true` if the [Sprite] only has fully transparent pixels.
    pub fn is_empty(&self) -> bool {
        self.image.pixels().all(|px| px.0[3] == 0)
    }
}

impl From<DynamicImage> for Sprite {
    fn from(val: DynamicImage) -> Self {
        Self {
            image: val.into_rgba8(),
        }
    }
}

impl From<RgbaImage> for Sprite {
    fn from(val: RgbaImage) -> Self {
        Self { image: val }
    }
}
