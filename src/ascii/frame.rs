use crate::{
    ascii::{Dictionary, Pixel},
    error::Result,
};
use bitcode::{Decode, Encode};
use regex::Regex;
use std::collections::HashSet;

#[derive(Encode, Decode, Debug)]
pub struct Frame {
    pub pixels: Vec<Pixel>,
}

impl Frame {
    pub fn new() -> Self {
        Frame { pixels: Vec::new() }
    }

    pub fn add_pixel(&mut self, pixel_id: u16, char_id: u8, color_id: u16) {
        self.pixels.push(Pixel::new(pixel_id, char_id, color_id))
    }

    pub fn capture_pixels(
        &mut self,
        content: &str,
        regex: &Regex,
        dictionary: &mut Dictionary,
    ) -> Result<()> {
        let mut pixel_id = 0;
        for line in content.lines() {
            for cap in regex.captures_iter(line) {
                let r = cap[1].parse::<u8>()?;
                let g = cap[2].parse::<u8>()?;
                let b = cap[3].parse::<u8>()?;
                let value = cap[4].chars().next().unwrap();

                let reduced_colors = Self::reduce_precision(r, g, b);

                let char_id = dictionary.get_or_insert_char(value);
                let color_id = dictionary.get_or_insert_color(reduced_colors);

                self.add_pixel(pixel_id, char_id, color_id);

                pixel_id += 1;
            }
        }

        Ok(())
    }

    fn reduce_precision(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
        //Reduce color precision to 565
        let r = (r >> 3) << 3;
        let g = (g >> 2) << 2;
        let b = (b >> 3) << 3;
        (r, g, b)
    }

    pub fn profile(&self) {
        let mut unique_char_ids = HashSet::new();
        let mut unique_color_ids = HashSet::new();
        let mut unique_char_colors = HashSet::new();

        for pixel in &self.pixels {
            unique_char_ids.insert(pixel.char_id);
            unique_color_ids.insert(pixel.color_id);
            unique_char_colors.insert((pixel.char_id, pixel.color_id));
        }

        let char_count = unique_char_ids.len();
        let color_count = unique_color_ids.len();
        let char_color_count = unique_char_colors.len();

        println!("Unique Char count: {}", char_count);
        println!("Unique Color count: {}", color_count);
        println!("Unique Char Color count: {}", char_color_count);
    }
}
