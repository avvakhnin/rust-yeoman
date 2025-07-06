use edict::{
    flow::FlowEntity,
    prelude::{Component, Res},
    view::View,
};
use rltk::{ColorPair, FontCharType, Point, PointF, RGBA};

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

enum Direction {
    Left,
    Right,
    Top,
    Down,
}

#[derive(Component)]
pub struct Mover {
    offset: f32,
    speed: f32,
    direction: Option<Direction>,
}

impl Mover {
    pub fn new() -> Self {
        Mover {
            offset: 0.,
            speed: 0.,
            direction: None,
        }
    }
    pub fn new_speed(speed: f32) -> Self {
        Mover {
            offset: 0.,
            speed,
            direction: None,
        }
    }
    pub fn move_left(&mut self) {
        self.direction = Some(Direction::Left)
    }
    pub fn move_right(&mut self) {
        self.direction = Some(Direction::Right)
    }
    pub fn move_top(&mut self) {
        self.direction = Some(Direction::Top)
    }
    pub fn move_down(&mut self) {
        self.direction = Some(Direction::Down)
    }
    fn stop(&mut self) {
        self.offset = 0.;
        self.direction = None;
    }
}

pub fn process_mover(v: View<(&mut Point, &mut Mover)>, d: Res<f32>) {
    for (p, m) in v {
        if m.direction.is_none() {
            return;
        }
        let no_half = m.offset < 0.5;
        println!("{} {}", m.offset, *d);
        m.offset += m.speed * *d;
        if no_half && m.offset >= 0.5 {
            match m.direction {
                Some(Direction::Left) => p.x -= 1,
                Some(Direction::Right) => p.x += 1,
                Some(Direction::Top) => p.y -= 1,
                Some(Direction::Down) => p.y += 1,
                None => unreachable!(),
            }
        }

        if m.offset > 1. {
            m.stop();
        }
    }
}
