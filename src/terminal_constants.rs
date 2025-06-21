use rltk::Point;

use crate::gui::layout::{ADDITIONAL_CONSOLE_DIMENSION, MAIN_CONSOLE_DIMENSION};

pub const MAP_Z_ORDER: usize = 0;
pub const CHAR_Z_ORDER: usize = 5000;

pub const INFO_Z_ORDER: usize = 0;

pub enum Consoles {
    Main,
    AdditionalVga,
}

impl Consoles {
    pub fn num(&self) -> usize {
        match *self {
            Consoles::Main => 0,
            Consoles::AdditionalVga => 1,
        }
    }
    pub fn dimensions(&self) -> Point {
        match *self {
            Consoles::Main => MAIN_CONSOLE_DIMENSION,
            Consoles::AdditionalVga => ADDITIONAL_CONSOLE_DIMENSION,
        }
    }
    pub fn font(&self) -> &str {
        match *self {
            Consoles::Main => "unicode_16x16.png",
            Consoles::AdditionalVga => "vga8x16.png",
        }
    }
    pub fn font_dimensions(&self) -> (usize, usize) {
        match *self {
            Consoles::Main => (16, 16),
            Consoles::AdditionalVga => (8, 16),
        }
    }
}
