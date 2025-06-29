use edict::{flow::FlowEntity, prelude::Component};
use rltk::{ColorPair, FontCharType, RGBA};

use crate::flow_timer::wait_pause_entity;

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
    pub fn new_blank() -> Self {
        Renderable {
            is_visible: false,
            glyph: 0,
            color: ColorPair::default(),
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

#[derive(Component)]
pub struct RenderStack {}

impl RenderStack {
    const RENDER_DATA: [(char, (u8, u8, u8)); 5] = [
        ('.', rltk::YELLOW),
        ('i', rltk::GREEN),
        ('|', rltk::GREEN),
        ('T', rltk::GREEN),
        ('T', rltk::YELLOW),
    ];
}

pub async fn rotate_render_stack(flow_entity: FlowEntity) {
    for i in RenderStack::RENDER_DATA {
        flow_entity.map(|mut er| {
            let r = er.get_mut::<&mut Renderable>().unwrap();
            r.is_visible = true;
            r.glyph = i.0 as u16;
            r.color.fg = RGBA::named(i.1);
        });
        wait_pause_entity(flow_entity, 1000f32).await;
    }
}

#[derive(Component)]
pub struct PlanJob {}
