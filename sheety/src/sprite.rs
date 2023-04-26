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
    pub fn load(path: &str) -> Result<Sprite> {
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
}

impl From<DynamicImage> for Sprite {
    fn from(val: DynamicImage) -> Self {
        Self {
            image: val.into_rgba8(),
        }
    }
}
