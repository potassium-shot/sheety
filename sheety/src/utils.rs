#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IVec2 {
    pub x: usize,
    pub y: usize,
}

impl IVec2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
