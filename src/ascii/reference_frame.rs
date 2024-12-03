use crate::ascii::{Frame, Pixel};
use std::collections::HashMap;

pub struct ReferenceFrame {
    pixels: HashMap<u16, Pixel>,
}

impl ReferenceFrame {
    pub fn new() -> Self {
        ReferenceFrame {
            pixels: HashMap::new(),
        }
    }

    pub fn apply_delta(&mut self, delta: &Frame) {
        for pixel in &delta.pixels {
            self.pixels.insert(pixel.id, pixel.clone());
        }
    }

    pub fn create_delta_frame(&self, frame: &Frame) -> Frame {
        let mut delta_pixels = Vec::new();

        for pixel in &frame.pixels {
            if let Some(ref_pixel) = self.pixels.get(&pixel.id) {
                if pixel.is_different(ref_pixel) {
                    delta_pixels.push(pixel.clone());
                }
            } else {
                delta_pixels.push(pixel.clone());
            }
        }

        Frame {
            pixels: delta_pixels,
        }
    }
}
