use bitcode::Encode;
use std::collections::HashMap;

#[derive(Debug, Encode)]
pub struct CharMap {
    pub chars: HashMap<u8, char>,
}

impl CharMap {
    pub fn new() -> Self {
        CharMap {
            chars: HashMap::new(),
        }
    }
    pub fn insert(&mut self, char_code: u8, char: char) {
        self.chars.insert(char_code, char);
    }
}

#[derive(Debug, Encode)]
pub struct ColorMap {
    pub colors: HashMap<u16, (u8, u8, u8)>,
}

impl ColorMap {
    pub fn new() -> Self {
        ColorMap {
            colors: HashMap::new(),
        }
    }

    pub fn insert(&mut self, color_code: u16, color: (u8, u8, u8)) {
        self.colors.insert(color_code, color);
    }
}

#[derive(Debug, Encode)]
pub struct Dictionary {
    pub chars: CharMap,
    reverse_chars: HashMap<char, u8>,
    pub colors: ColorMap,
    reverse_colors: HashMap<(u8, u8, u8), u16>,
    char_interval: u8,
    color_interval: u16,
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {
            chars: CharMap::new(),
            reverse_chars: HashMap::new(),
            colors: ColorMap::new(),
            reverse_colors: HashMap::new(),
            char_interval: 0,
            color_interval: 0,
        }
    }

    pub fn get_or_insert_char(&mut self, char: char) -> u8 {
        if let Some(char_code) = self.reverse_chars.get(&char) {
            *char_code
        } else {
            self.char_interval += 1;
            self.insert_char(self.char_interval, char);
            self.char_interval
        }
    }

    pub fn get_or_insert_color(&mut self, color: (u8, u8, u8)) -> u16 {
        if let Some(color_code) = self.reverse_colors.get(&color) {
            *color_code
        } else {
            self.color_interval += 1;
            self.insert_color(self.color_interval, color);
            self.color_interval
        }
    }

    fn insert_char(&mut self, char_code: u8, char: char) {
        self.chars.insert(char_code, char);
        self.reverse_chars.insert(char, char_code);
    }

    fn insert_color(&mut self, color_code: u16, color: (u8, u8, u8)) {
        self.colors.insert(color_code, color);
        self.reverse_colors.insert(color, color_code);
    }
}
