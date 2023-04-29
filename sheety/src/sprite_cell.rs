use crate::{sprite::Sprite, utils::IVec2};

/// Represents a cell of a [SpriteSheet][crate::SpriteSheet].
/// It can either be [Empty][SpriteCell::Empty], or contain a [Sprite][SpriteCell::Sprite].
#[derive(Default, Debug, Clone)]
pub enum SpriteCell {
    /// A cell that contains a [Sprite].
    Sprite(Sprite),
    #[default]
    /// An empty cell. These cells are not included when converting a [SpriteSheet][crate::SpriteSheet] to an
    /// [UnorderedSpriteSheet][crate::UnorderedSpriteSheet].
    Empty,
}

impl SpriteCell {
    /// Gets the size of the [Sprite] contained in a [SpriteCell::Sprite] variant, or [None] if it is
    /// an [SpriteCell::Empty] variant.
    pub fn size(&self) -> Option<IVec2> {
        match self {
            Self::Sprite(sprite) => Some(sprite.size()),
            Self::Empty => None,
        }
    }

    /// Returns `true` if this is a [SpriteCell::Sprite] variant.
    pub fn is_sprite(&self) -> bool {
        match self {
            Self::Sprite(_) => true,
            Self::Empty => false,
        }
    }

    /// Returns `true` if this is an [SpriteCell::Empty] variant.
    pub fn is_empty(&self) -> bool {
        !self.is_sprite()
    }

    /// Returns the underlying [Sprite] of a [SpriteCell::Sprite] variant, or [None]
    /// if it is an [SpriteCell::Empty] variant.
    pub fn sprite(self) -> Option<Sprite> {
        match self {
            Self::Sprite(sprite) => Some(sprite),
            Self::Empty => None,
        }
    }
}
