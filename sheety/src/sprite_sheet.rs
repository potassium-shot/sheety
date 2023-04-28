use std::{path::Path, vec};

use image::{GenericImage, GenericImageView, RgbaImage};

use crate::{
    error::{Error, Result},
    sprite_cell::SpriteCell,
    unordered_sprite_sheet::UnorderedSpriteSheet,
    utils::IVec2,
    Distribution, Sprite,
};

const EXPECT_MSG_OUTOFBOUNDS: &str =
    "coords have already been checked to be inbounds and should be inbounds by this point; \
	if this panic occurs then the SpriteSheet.size is not in sync with the size of the inner \
	SpriteSheet.cells vector";

const EXPECT_MSG_SHEET_FULL: &str =
    "Distribution::get_min_size should always return a size that fits";

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
                temp_vec.reserve_exact(size.1);

                for _ in 0..size.1 {
                    temp_vec.push(vec![SpriteCell::Empty; size.0]);
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
            .get(coords.1)
            .ok_or(Error::OutOfRange {
                max: self.size,
                provided: coords,
            })?
            .get(coords.0)
            .ok_or(Error::OutOfRange {
                max: self.size,
                provided: coords,
            })?)
    }

    pub fn get_cell_mut(&mut self, coords: IVec2) -> Result<&mut SpriteCell> {
        Ok(self
            .cells
            .get_mut(coords.1)
            .ok_or(Error::OutOfRange {
                max: self.size,
                provided: coords,
            })?
            .get_mut(coords.0)
            .ok_or(Error::OutOfRange {
                max: self.size,
                provided: coords,
            })?)
    }

    pub fn set_cell(&mut self, coords: IVec2, cell: SpriteCell) -> Result<SpriteCell> {
        if coords.0 > self.size.0 || coords.1 > self.size.1 {
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
                .get_mut(coords.1)
                .expect(EXPECT_MSG_OUTOFBOUNDS)
                .get_mut(coords.0)
                .expect(EXPECT_MSG_OUTOFBOUNDS),
            cell,
        ))
    }

    pub fn cells(&self) -> IterCells {
        IterCells::new(self)
    }

    pub fn cells_mut(&mut self) -> IterCellsMut {
        IterCellsMut::new(self)
    }

    pub fn into_unordered(self) -> Result<UnorderedSpriteSheet> {
        UnorderedSpriteSheet::new(self.into_iter().filter_map(|item| item.sprite()).collect())
    }

    pub fn push_sprite(&mut self, sprite: Sprite) -> Result<()> {
        *(self
            .cells_mut()
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
        sheet.push_sprites(sprites).expect(EXPECT_MSG_SHEET_FULL);
        sheet
    }

    pub fn concat<I>(sprites: I, distribution: Distribution) -> Result<Self>
    where
        I: Iterator<Item = UnorderedSpriteSheet>,
    {
        let list: Vec<UnorderedSpriteSheet> = sprites.collect();

        if list.len() == 0 {
            return Err(Error::EmptyIterator);
        }

        let size = list[0].size();

        let mut len = 0;

        for unordered in list.iter() {
            if unordered.size() != size {
                return Err(Error::MismatchedSpriteSize {
                    required: size,
                    provided: unordered.size(),
                });
            }

            len += unordered.len();
        }

        let mut sheet = Self::new(distribution.get_min_size(len), size);

        for unordered in list {
            sheet.push_sprites(unordered).expect(EXPECT_MSG_SHEET_FULL);
        }

        Ok(sheet)
    }

    pub fn from_image_div(sprite: Sprite, divisions: IVec2) -> Self {
        let cell_size = (sprite.size().0 / divisions.0, sprite.size().1 / divisions.1);

        Self::from_image(sprite, divisions, cell_size)
    }

    pub fn from_image_cell_size(sprite: Sprite, cell_size: IVec2) -> Self {
        let divisions = (sprite.size().0 / cell_size.0, sprite.size().1 / cell_size.1);

        Self::from_image(sprite, divisions, cell_size)
    }

    fn from_image(sprite: Sprite, divisions: IVec2, cell_size: IVec2) -> Self {
        let image = sprite.into_image();

        let mut sheet = Self::new(divisions, cell_size);

        for x in 0..divisions.0 {
            for y in 0..divisions.1 {
                let sub_sprite: Sprite = image
                    .view(
                        (x * cell_size.0) as u32,
                        (y * cell_size.1) as u32,
                        cell_size.0 as u32,
                        cell_size.1 as u32,
                    )
                    .to_image()
                    .into();

                sheet
                    .set_cell(
                        (x, y),
                        if sub_sprite.is_empty() {
                            SpriteCell::Empty
                        } else {
                            SpriteCell::Sprite(sub_sprite)
                        },
                    )
                    .expect(EXPECT_MSG_OUTOFBOUNDS);
            }
        }

        sheet
    }

    pub fn into_image(mut self) -> RgbaImage {
        let mut final_image = RgbaImage::new(
            (self.cell_size.0 * self.size.0) as u32,
            (self.cell_size.1 * self.size.1) as u32,
        );

        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                match std::mem::replace(
                    self.cells[y].get_mut(x).expect(EXPECT_MSG_OUTOFBOUNDS),
                    SpriteCell::Empty,
                ) {
                    SpriteCell::Sprite(sprite) => final_image
                        .copy_from(
                            &sprite.into_image(),
                            (x * self.cell_size.0) as u32,
                            (y * self.cell_size.1) as u32,
                        )
                        .expect("image should have already been checked to be of the right size at insertion time"),
                    SpriteCell::Empty => (),
                }
            }
        }

        final_image
    }

    pub fn load_div<P>(path: P, divisions: IVec2) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(Self::from_image_div(Sprite::load(path)?, divisions))
    }

    pub fn load_cell_size<P>(path: P, cell_size: IVec2) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(Self::from_image_cell_size(Sprite::load(path)?, cell_size))
    }

    pub fn save<P>(self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        self.into_image()
            .save(path)
            .map_err(|err| Error::ImageError(err))
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
        if self.next_index >= self.sheet.size.0 {
            self.current_line += 1;
            self.next_index = 0;

            if self.current_line >= self.sheet.size.1 {
                return None;
            }
        }

        let next = self
            .sheet
            .cells
            .get(self.current_line)
            .expect(EXPECT_MSG_OUTOFBOUNDS)
            .get(self.next_index)
            .expect(EXPECT_MSG_OUTOFBOUNDS);

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
        if self.next_index >= self.sheet.size.0 {
            self.current_line += 1;
            self.next_index = 0;

            if self.current_line >= self.sheet.size.1 {
                return None;
            }
        }

        let next = self
            .sheet
            .cells
            .get_mut(self.current_line)
            .expect(EXPECT_MSG_OUTOFBOUNDS)
            .get_mut(self.next_index)
            .expect(EXPECT_MSG_OUTOFBOUNDS);

        self.next_index += 1;
        Some(unsafe { std::mem::transmute(next) })
    }
}
