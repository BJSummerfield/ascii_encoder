use crate::{ascii::Pixel, error::Result};
use bitcode::{Decode, Encode};
use regex::Regex;

#[derive(Encode, Decode, Debug)]
pub struct Frame {
    pub pixels: Vec<Pixel>,
}

impl Frame {
    pub fn new() -> Self {
        Frame { pixels: Vec::new() }
    }

    pub fn add_pixel(&mut self, x: usize, y: usize, value: char, color: (u8, u8, u8)) {
        self.pixels.push(Pixel::new(x, y, value, color));
    }

    pub fn capture_pixels(&mut self, content: &str, regex: &Regex, width: usize) -> Result<()> {
        let mut x = 0;
        let mut y = 0;

        for line in content.lines() {
            for cap in regex.captures_iter(line) {
                let r = cap[1].parse::<u8>()?;
                let g = cap[2].parse::<u8>()?;
                let b = cap[3].parse::<u8>()?;
                let value = cap[4].chars().next().unwrap();

                self.add_pixel(x, y, value, (r, g, b));

                x += 1;
                if x >= width {
                    x = 0;
                    y += 1;
                }
            }
        }

        Ok(())
    }
}
