use crate::utils::IVec2;

/// Reprensents a constraint to follow when building [SpriteSheet][crate::SpriteSheet] from an
/// [UnorderedSpriteSheet][crate::UnorderedSpriteSheet].
#[derive(Debug, Clone)]
pub enum Distribution {
    /// Requires that the [SpriteSheet][crate::SpriteSheet] has a fixed amount of columns.
    /// The number of lines will then depend on the amount of [Sprite][crate::Sprite]s in the sheet.
    FixedColumns(usize),
    /// Requires that the [SpriteSheet][crate::SpriteSheet] has a fixed amount of lines.
    /// The number of columns will then depend on the amount of [Sprite][crate::Sprite]s in the sheet.
    FixedLines(usize),
    /// Requires that the [SpriteSheet][crate::SpriteSheet] attempts to pack all sprites in a sructure close
    /// to a square, with as little empty space as possible. The [bool] parameter should be `true` if the algorithm
    /// should favoritise column length, or `false` to favoritise line length - for cases where one side must be
    /// larger than the other.
    Packed(bool),
}

impl Distribution {
    /// Returns the minimum possible size a [SpriteSheet][crate::SpriteSheet] should have, whilst respecting
    /// its `sprite_count` and this [Distribution].
    pub fn get_min_size(&self, sprite_count: usize) -> IVec2 {
        match self {
            Self::FixedColumns(columns) => (
                *columns,
                f32::ceil(sprite_count as f32 / *columns as f32) as usize,
            ),
            Self::FixedLines(lines) => (
                f32::ceil(sprite_count as f32 / *lines as f32) as usize,
                *lines,
            ),
            Self::Packed(lines_prio) => {
                let base = f32::sqrt(sprite_count as f32);

                if *lines_prio {
                    (f32::ceil(base) as usize, f32::floor(base + 0.5) as usize)
                } else {
                    (f32::floor(base + 0.5) as usize, f32::ceil(base) as usize)
                }
            }
        }
    }
}
