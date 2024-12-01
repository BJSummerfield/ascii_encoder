use bitcode::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    value: char,
    color: (u8, u8, u8),
}

impl Pixel {
    pub fn new(x: usize, y: usize, value: char, color: (u8, u8, u8)) -> Self {
        Pixel { x, y, value, color }
    }

    pub fn is_different(&self, other: &Pixel) -> bool {
        self.value != other.value || self.color != other.color
    }
}
