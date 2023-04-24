use std::vec;

use crate::Sprite;

#[derive(Default, Debug, Clone)]
pub struct UnorderedSpriteSheet {
    sprites: Vec<Sprite>,
}

impl UnorderedSpriteSheet {
    pub fn new(sprites: Vec<Sprite>) -> Self {
        Self { sprites }
    }
}

impl IntoIterator for UnorderedSpriteSheet {
    type Item = Sprite;

    type IntoIter = vec::IntoIter<Sprite>;

    fn into_iter(self) -> Self::IntoIter {
        self.sprites.into_iter()
    }
}
