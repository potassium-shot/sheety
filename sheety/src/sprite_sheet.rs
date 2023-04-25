use std::vec;

use crate::{
    error::{Error, Result},
    sprite_cell::SpriteCell,
    unordered_sprite_sheet::UnorderedSpriteSheet,
    utils::IVec2,
    Distribution, Sprite,
};

const PANIC_MSG_OUTOFBOUNDS: &str =
    "coords have already been checked to be inbounds and should be inbounds by this point; \
	if this panic occurs then the SpriteSheet.size is not in sync with the size of the inner \
	SpriteSheet.cells vector";

pub struct SpriteSheet {
    cells: Vec<Vec<SpriteCell>>, // Vector of lines, each line is a vector of cells
    size: IVec2,
    cell_size: IVec2,
}

impl SpriteSheet {
    pub fn new(size: IVec2, cell_size: IVec2) -> SpriteSheet {
        SpriteSheet {
            cells: {
                let mut temp_vec = Vec::new(); // vector of lines
                temp_vec.reserve_exact(size.y);

                for _ in 0..size.y {
                    temp_vec.push(vec![SpriteCell::Empty; size.x]);
                }

                temp_vec
            },
            size,
            cell_size,
        }
    }

    #[inline(always)]
    pub fn size(&self) -> IVec2 {
        self.size
    }

    pub fn get_cell(&self, coords: IVec2) -> Result<&SpriteCell> {
        Ok(self
            .cells
            .get(coords.y)
            .ok_or(Error::OutOfRange {
                max: self.size,
                provided: coords,
            })?
            .get(coords.x)
            .ok_or(Error::OutOfRange {
                max: self.size,
                provided: coords,
            })?)
    }

    pub fn get_cell_mut(&mut self, coords: IVec2) -> Result<&mut SpriteCell> {
        Ok(self
            .cells
            .get_mut(coords.y)
            .ok_or(Error::OutOfRange {
                max: self.size,
                provided: coords,
            })?
            .get_mut(coords.x)
            .ok_or(Error::OutOfRange {
                max: self.size,
                provided: coords,
            })?)
    }

    pub fn set_cell(&mut self, coords: IVec2, cell: SpriteCell) -> Result<SpriteCell> {
        if coords.x > self.size.x || coords.y > self.size.y {
            return Err(Error::OutOfRange {
                max: self.size,
                provided: coords,
            });
        }

        if let Some(size) = cell.size() {
            if size != self.cell_size {
                return Err(Error::MismatchedSpriteSize {
                    required: self.cell_size,
                    provided: size,
                });
            }
        }

        Ok(std::mem::replace(
            self.cells
                .get_mut(coords.y)
                .expect(PANIC_MSG_OUTOFBOUNDS)
                .get_mut(coords.x)
                .expect(PANIC_MSG_OUTOFBOUNDS),
            cell,
        ))
    }

    pub fn iter_cells(&self) -> IterCells {
        IterCells::new(self)
    }

    pub fn iter_cells_mut(&mut self) -> IterCellsMut {
        IterCellsMut::new(self)
    }

    pub fn into_unordered(self) -> Result<UnorderedSpriteSheet> {
        UnorderedSpriteSheet::new(self.into_iter().filter_map(|item| item.sprite()).collect())
    }

    pub fn push_sprite(&mut self, sprite: Sprite) -> Result<()> {
        *(self
            .iter_cells_mut()
            .find(|item| item.is_empty())
            .ok_or(Error::SheetFull { amount_fitted: 0 })?) = SpriteCell::Sprite(sprite);

        Ok(())
    }

    pub fn push_sprites(&mut self, sprites: UnorderedSpriteSheet) -> Result<()> {
        let mut fitted = 0;

        for sprite in sprites {
            self.push_sprite(sprite).map_err(|_| Error::SheetFull {
                amount_fitted: fitted,
            })?;

            fitted += 1;
        }

        Ok(())
    }

    pub fn from_unordered(sprites: UnorderedSpriteSheet, distribution: Distribution) -> Self {
        let mut sheet = Self::new(distribution.get_min_size(sprites.len()), sprites.size());
        sheet
            .push_sprites(sprites)
            .expect("Distribution::get_size should always return a size that fits");
        sheet
    }
}

impl IntoIterator for SpriteSheet {
    type Item = SpriteCell;

    type IntoIter = IntoIterCells;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

// IntoIter

pub struct IntoIterCells {
    sheet_iter: vec::IntoIter<Vec<SpriteCell>>,
    line_iter: Option<Vec<SpriteCell>>,
}

impl IntoIterCells {
    fn new(sheet: SpriteSheet) -> Self {
        Self {
            sheet_iter: sheet.cells.into_iter(),
            line_iter: None,
        }
    }
}

impl Iterator for IntoIterCells {
    type Item = SpriteCell;

    fn next(&mut self) -> Option<Self::Item> {
        // take the next line (if none return none)
        // take the next cell on the line (if none return none)

        if match self.line_iter {
            Some(ref it) if it.len() == 0 => true, // is it empty
            None => true,                          // or is it None
            _ => false,
        } {
            self.line_iter = self.sheet_iter.next();
        }

        if let Some(ref mut vec) = self.line_iter {
            Some(vec.remove(0))
        } else {
            None
        }
    }
}

// Iter

pub struct IterCells<'a> {
    sheet: &'a SpriteSheet,
    next_index: usize,
    current_line: usize,
}

impl<'a> IterCells<'a> {
    fn new(sheet: &'a SpriteSheet) -> Self {
        Self {
            sheet,
            next_index: 0,
            current_line: 0,
        }
    }
}

impl<'a> Iterator for IterCells<'a> {
    type Item = &'a SpriteCell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.sheet.size.x {
            self.current_line += 1;
            self.next_index = 0;

            if self.current_line >= self.sheet.size.y {
                return None;
            }
        }

        let next = self
            .sheet
            .cells
            .get(self.current_line)
            .expect(PANIC_MSG_OUTOFBOUNDS)
            .get(self.next_index)
            .expect(PANIC_MSG_OUTOFBOUNDS);

        self.next_index += 1;
        Some(next)
    }
}

pub struct IterCellsMut<'a> {
    sheet: &'a mut SpriteSheet,
    next_index: usize,
    current_line: usize,
}

impl<'a> IterCellsMut<'a> {
    fn new(sheet: &'a mut SpriteSheet) -> Self {
        Self {
            sheet,
            next_index: 0,
            current_line: 0,
        }
    }
}

impl<'a> Iterator for IterCellsMut<'a> {
    type Item = &'a mut SpriteCell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.sheet.size.x {
            self.current_line += 1;
            self.next_index = 0;

            if self.current_line >= self.sheet.size.y {
                return None;
            }
        }

        let next = self
            .sheet
            .cells
            .get_mut(self.current_line)
            .expect(PANIC_MSG_OUTOFBOUNDS)
            .get_mut(self.next_index)
            .expect(PANIC_MSG_OUTOFBOUNDS);

        self.next_index += 1;
        Some(unsafe { std::mem::transmute(next) })
    }
}
