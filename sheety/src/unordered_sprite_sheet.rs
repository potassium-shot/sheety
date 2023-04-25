use std::vec;

use crate::{
    error::{Error, Result},
    IVec2, Sprite,
};

#[derive(Default, Debug, Clone)]
pub struct UnorderedSpriteSheet {
    sprites: Vec<Sprite>,
    size: IVec2,
}

impl UnorderedSpriteSheet {
    pub fn new(sprites: Vec<Sprite>) -> Result<Self> {
        let mut sprites_iter = sprites.iter();

        let size = sprites_iter
            .next()
            .ok_or(Error::EmptyUnorderedSpriteSheet)?
            .size();

        for sprite in sprites_iter {
            if sprite.size() != size {
                return Err(Error::MismatchedSpriteSize {
                    required: size,
                    provided: sprite.size(),
                });
            }
        }

        Ok(Self { sprites, size })
    }

    pub fn len(&self) -> usize {
        self.sprites.len()
    }

    pub fn size(&self) -> IVec2 {
        self.size
    }
}

impl IntoIterator for UnorderedSpriteSheet {
    type Item = Sprite;

    type IntoIter = vec::IntoIter<Sprite>;

    fn into_iter(self) -> Self::IntoIter {
        self.sprites.into_iter()
    }
}
