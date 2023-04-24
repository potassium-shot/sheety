use crate::{sprite::Sprite, utils::IVec2};

#[derive(Default, Debug, Clone)]
pub enum SpriteCell {
    Sprite(Sprite),
    #[default]
    Empty,
}

impl SpriteCell {
    pub fn size(&self) -> Option<IVec2> {
        match self {
            Self::Sprite(sprite) => Some(sprite.size()),
            Self::Empty => None,
        }
    }

    pub fn is_sprite(&self) -> bool {
        match self {
            Self::Sprite(_) => true,
            Self::Empty => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.is_sprite()
    }

    pub fn sprite(self) -> Option<Sprite> {
        match self {
            Self::Sprite(sprite) => Some(sprite),
            Self::Empty => None,
        }
    }
}
