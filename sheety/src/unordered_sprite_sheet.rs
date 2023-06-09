use std::vec;

use crate::{
    error::{Error, Result},
    utils::IVec2,
    Sprite,
};

/// An [UnorderedSpriteSheet] encapsulate a list of [Sprite]s. It can be optained from a [SpriteSheet][crate::SpriteSheet]
/// by using [SpriteSheet::into_unordered][crate::SpriteSheet::into_unordered], or can be created from a [Vec] of [Sprite]s.
#[derive(Default, Debug, Clone)]
pub struct UnorderedSpriteSheet {
    sprites: Vec<Sprite>,
    size: IVec2,
}

impl UnorderedSpriteSheet {
    /// Creates an [UnorderedSpriteSheet] from a [Vec] of [Sprite]s.
    ///
    /// # Errors
    ///
    /// - Will return [Error::EmptyUnorderedSpriteSheet] if the supplied [Vec] is empty.
    /// The [UnorderedSpriteSheet] needs at least one [Sprite] to determine its size.
    /// - Will return [Error::MismatchedSpriteSize] if the supplied [Sprite]s don't each have the same size.
    ///
    /// # Examples
    ///
    /// ```
    /// let sprite_1 = Sprite::load("path/to/a/sprite.png").unwrap();
    /// let sprite_2 = Sprtie::load("path/to/another/sprite.png").unwrap();
    ///
    /// let sprites = UnorderedSpriteSheet::new(vec![sprite_1, sprite_2]);
    ///
    /// assert_eq!(sprites.len(), 2);
    /// ```
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

    /// Gets the number of [Sprite]s in the [UnorderedSpriteSheet].
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.sprites.len()
    }

    /// Returns `true` if there are no [Sprite]s in the [UnorderedSpriteSheet].
    pub fn is_empty(&self) -> bool {
        self.sprites.is_empty()
    }

    /// Gets the size of [Sprite] within the [UnorderedSpriteSheet].
    #[inline(always)]
    pub fn size(&self) -> IVec2 {
        self.size
    }

    /// Gets an immutable reference to the inner vector of sprites.
    #[inline(always)]
    pub fn inner(&self) -> &Vec<Sprite> {
        &self.sprites
    }

    /// Gets a mutable reference to the inner vector of [Sprite]s.
    #[inline(always)]
    pub fn inner_mut(&mut self) -> &mut Vec<Sprite> {
        &mut self.sprites
    }

    /// Consumes this [UnorderedSpriteSheet] and returns its underlying vector of [Sprite]s.
    pub fn into_inner(self) -> Vec<Sprite> {
        self.sprites
    }
}

impl IntoIterator for UnorderedSpriteSheet {
    type Item = Sprite;

    type IntoIter = vec::IntoIter<Sprite>;

    fn into_iter(self) -> Self::IntoIter {
        self.sprites.into_iter()
    }
}
