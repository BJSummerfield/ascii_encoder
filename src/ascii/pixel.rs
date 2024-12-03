use bitcode::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone)]
pub struct Pixel {
    pub id: u16,
    pub char_id: u8,
    pub color_id: u16,
}

impl Pixel {
    pub fn new(id: u16, char_id: u8, color_id: u16) -> Self {
        Pixel {
            id,
            char_id,
            color_id,
        }
    }

    pub fn is_different(&self, other: &Pixel) -> bool {
        self.char_id != other.char_id || self.color_id != other.color_id
    }
}
