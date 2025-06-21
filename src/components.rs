use edict::prelude::Component;
use rltk::{ColorPair, FontCharType, RGBA};

#[derive(Component)]
pub struct Renderable {
    pub is_visible: bool,
    pub glyph: FontCharType,
    pub color: ColorPair,
}

impl Renderable {
    pub fn new(glyph: char, color: (u8, u8, u8)) -> Self {
        Renderable {
            is_visible: true,
            glyph: glyph as u16,
            color: ColorPair::new(RGBA::named(color), rltk::BLACK),
        }
    }
    pub fn new_bg(glyph: char, color: (u8, u8, u8), background: (u8, u8, u8)) -> Self {
        Renderable {
            is_visible: true,
            glyph: glyph as u16,
            color: ColorPair::new(RGBA::named(color), RGBA::named(background)),
        }
    }
}
