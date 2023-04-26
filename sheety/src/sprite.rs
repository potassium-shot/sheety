use std::path::Path;

use image::{DynamicImage, RgbaImage};

use crate::{
    error::{Error, Result},
    utils::IVec2,
};

#[derive(Debug, Clone)]
pub struct Sprite {
    image: RgbaImage,
}

impl Sprite {
    pub fn load<P>(path: P) -> Result<Sprite>
    where
        P: AsRef<Path>,
    {
        Ok(Self::from(
            image::open(path).map_err(|err| Error::ImageError(err))?,
        ))
    }

    pub fn size(&self) -> IVec2 {
        IVec2::new(self.image.width() as usize, self.image.height() as usize)
    }

    pub fn into_image(self) -> RgbaImage {
        self.image
    }

    pub fn is_empty(&self) -> bool {
        self.image
            .pixels()
            .all(|px| (px.0[0] & px.0[1] & px.0[2] & px.0[3]) == 0)
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
