use std::ops::{Add, Div, Mul, Sub};

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

macro_rules! impl_operator {
    ($tr:ty, $func:ident, $op:tt) => {
        impl $tr for IVec2 {
            type Output = IVec2;

            fn $func(self, rhs: Self) -> Self::Output {
                Self::new(self.x $op rhs.x, self.y $op rhs.y)
            }
        }
    };
}

impl_operator!(Add, add, +);
impl_operator!(Sub, sub, -);
impl_operator!(Mul, mul, *);
impl_operator!(Div, div, /);
