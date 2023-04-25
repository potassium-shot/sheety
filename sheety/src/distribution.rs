use crate::IVec2;

#[derive(Debug, Clone)]
pub enum Distribution {
    FixedColumns(usize),
    FixedLines(usize),
    Packed(bool),
}

impl Distribution {
    pub fn get_min_size(&self, sprite_count: usize) -> IVec2 {
        match self {
            Self::FixedColumns(columns) => IVec2::new(
                *columns,
                f32::ceil(sprite_count as f32 / *columns as f32) as usize,
            ),
            Self::FixedLines(lines) => IVec2::new(
                f32::ceil(sprite_count as f32 / *lines as f32) as usize,
                *lines,
            ),
            Self::Packed(lines_prio) => {
                let base = f32::sqrt(sprite_count as f32) + 0.5;

                if *lines_prio {
                    IVec2::new(f32::ceil(base) as usize, f32::floor(base) as usize)
                } else {
                    IVec2::new(f32::floor(base) as usize, f32::ceil(base) as usize)
                }
            }
        }
    }
}
