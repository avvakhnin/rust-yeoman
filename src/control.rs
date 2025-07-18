use core::fmt;
use std::{
    cmp::{max, min},
    env::var,
    ops::ControlFlow,
};

use crate::{
    State,
    ambience::map::MAP_BORDER,
    components::{PlanJob, Renderable, rotate_render_stack},
    flow_timer::wait_pause_entity,
    math::QuasiRect,
    spawn::{create_plan_job, create_plant_flow},
};
use edict::{entity::EntityId, flow::FlowWorld, query::Entities, world::World};
use rltk::{ColorPair, Point, ROYALBLUE4, Rect, Rltk, VirtualKeyCode};

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ControlMode {
    Player,
    Cursor,
    Corner,
    Auto,
}

impl fmt::Display for ControlMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
impl ControlMode {
    fn switch_control_mode(gs: &mut State) {
        gs.mode = match &gs.mode {
            ControlMode::Player => ControlMode::Cursor,
            ControlMode::Cursor => ControlMode::Corner,
            ControlMode::Corner => ControlMode::Player,
            ControlMode::Auto => ControlMode::Player,
        };
        if let ControlMode::Cursor = gs.mode {
            let pos = gs.world.view::<&Point>();
            let pos = pos.try_get(gs.player_id).expect("Player does not exist");
            let mut rect = gs.world.view::<&mut Rect>();
            let rect = rect
                .try_get_mut(gs.cursor_id)
                .expect("Cursor does not exist");
            rect.x1 = pos.x;
            rect.x2 = pos.x;
            rect.y1 = pos.y;
            rect.y2 = pos.y;
        }
    }

    fn switch_auto_mode(gs: &mut State) {
        gs.mode = ControlMode::Auto
    }

    fn process_moving(gs: &State, delta_x: i32, delta_y: i32) {
        match gs.mode {
            ControlMode::Player => Self::try_move_player(gs, delta_x, delta_y),
            ControlMode::Cursor => Self::try_move_cursor_start(gs, delta_x, delta_y),
            ControlMode::Corner => Self::try_move_cursor_end(gs, delta_x, delta_y),
            ControlMode::Auto => (),
        }
    }

    fn process_action(gs: &mut State) {
        match gs.mode {
            ControlMode::Player => start_future(gs),
            ControlMode::Cursor => Self::process_action_on_cursor(gs),
            ControlMode::Corner => Self::process_action_on_cursor(gs),
            ControlMode::Auto => (),
        }
    }

    fn process_action_on_cursor(gs: &mut State) {
        let &mut rect = gs
            .world
            .view::<&mut Rect>()
            .try_get_mut(gs.cursor_id)
            .expect("Cursor does not exist");
        rect.envelop_rect()
            .for_each(|p| create_plan_job(&mut gs.world, p, gs.player_id));
    }

    fn try_move_player(gs: &State, delta_x: i32, delta_y: i32) {
        let mut pos = gs.world.view::<&mut Point>();

        let pos = pos
            .try_get_mut(gs.player_id)
            .expect("Player does not exist");

        pos.x = min(MAP_BORDER.x2 - 1, max(MAP_BORDER.x1, pos.x + delta_x));
        pos.y = min(MAP_BORDER.y2 - 1, max(MAP_BORDER.y1, pos.y + delta_y));
    }

    fn try_move_cursor_start(gs: &State, delta_x: i32, delta_y: i32) {
        let mut rect = gs.world.view::<&mut Rect>();

        let rect = rect
            .try_get_mut(gs.cursor_id)
            .expect("Cursor does not exist");

        rect.x1 = min(MAP_BORDER.x2 - 1, max(MAP_BORDER.x1, rect.x1 + delta_x));
        rect.y1 = min(MAP_BORDER.y2 - 1, max(MAP_BORDER.y1, rect.y1 + delta_y));
    }

    fn try_move_cursor_end(gs: &State, delta_x: i32, delta_y: i32) {
        let mut rect = gs.world.view::<&mut Rect>();

        let rect = rect
            .try_get_mut(gs.cursor_id)
            .expect("Cursor does not exist");

        rect.x2 = min(MAP_BORDER.x2 - 1, max(MAP_BORDER.x1, rect.x2 + delta_x));
        rect.y2 = min(MAP_BORDER.y2 - 1, max(MAP_BORDER.y1, rect.y2 + delta_y));
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened

        Some(key) => match key {
            VirtualKeyCode::Left => ControlMode::process_moving(gs, -1, 0),
            VirtualKeyCode::Right => ControlMode::process_moving(gs, 1, 0),
            VirtualKeyCode::Up => ControlMode::process_moving(gs, 0, -1),
            VirtualKeyCode::Down => ControlMode::process_moving(gs, 0, 1),
            VirtualKeyCode::Return => ControlMode::process_action(gs),
            VirtualKeyCode::Space => ControlMode::switch_control_mode(gs),
            VirtualKeyCode::A => ControlMode::switch_auto_mode(gs),
            _ => {}
        },
    }
}

fn start_future(gs: &mut State) {
    let epoch_id = gs.world.epoch();
    let player_id = gs.player_id.clone();
    let pos = gs
        .world
        .view::<&Point>()
        .try_get(player_id)
        .expect("Player does not exist")
        .clone();
    gs.world.spawn_flow_for(player_id, async move |fe| {
        wait_pause_entity(fe, 1000f32).await;
        let is_modified = fe.world().map(move |w| {
            w.view::<Entities>()
                .modified::<Point>(epoch_id)
                .try_get(player_id)
                .is_ok()
        });
        if is_modified {
            return;
        }
        create_plant_flow(&mut fe.world(), pos);
    });
}
