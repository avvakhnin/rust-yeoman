use edict::{
    flow::FlowEntity,
    prelude::{Component, Res},
    view::View,
};
use rltk::{ColorPair, FontCharType, Point, RGBA, RandomNumberGenerator};

use crate::{ambience::map::MAP_BORDER, flow_timer::wait_pause_entity};

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
    pub fn new_speed(speed: f32) -> Self {
        Mover {
            offset: 0.,
            speed,
            direction: None,
        }
    }
    pub fn move_left(&mut self) {
        self.direction.get_or_insert(Direction::Left);
    }
    pub fn move_right(&mut self) {
        self.direction.get_or_insert(Direction::Right);
    }
    pub fn move_top(&mut self) {
        self.direction.get_or_insert(Direction::Top);
    }
    pub fn move_down(&mut self) {
        self.direction.get_or_insert(Direction::Down);
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
        m.offset += m.speed * *d;
        if no_half && m.offset >= 0.5 {
            let (delta_x, delta_y) = match &m.direction {
                None => unreachable!(),
                Some(direction) => match direction {
                    Direction::Left => (-1, 0),
                    Direction::Right => (1, 0),
                    Direction::Top => (0, -1),
                    Direction::Down => (0, 1),
                },
            };
            p.x = (p.x + delta_x).clamp(MAP_BORDER.x1, MAP_BORDER.x2 - 1);
            p.y = (p.y + delta_y).clamp(MAP_BORDER.y1, MAP_BORDER.y2 - 1);
        }
        if m.offset > 1. {
            m.stop();
        }
    }
}

#[derive(Component)]
pub struct HareBrain {
    last_choise: i32,
    rand: RandomNumberGenerator,
}

impl HareBrain {
    pub fn new() -> Self {
        let mut rand = RandomNumberGenerator::new();
        let last_choise = rand.range(0, 4);
        HareBrain { last_choise, rand }
    }
}

pub fn process_hare_brain(v: View<(&mut Mover, &mut HareBrain)>) {
    for (m, b) in v {
        if m.direction.is_some() {
            return;
        }
        let direction = if b.rand.rand::<f32>() < 0.75 {
            b.last_choise
        } else {
            b.rand.range(0, 4)
        };
        b.last_choise = direction;
        match direction {
            0 => m.move_left(),
            1 => m.move_right(),
            2 => m.move_top(),
            3 => m.move_down(),
            _ => {}
        }
    }
}
