use std::cmp::{max, min};

use rltk::{DrawBatch, Point, Rect};

use crate::{
    State,
    ambience::map::{MAP_BORDER, MapField, MapTile},
    components::Renderable,
    gui::layout::MAIN_VIEW_POSITION,
    terminal_constants::{CHAR_Z_ORDER, MAP_Z_ORDER},
};

const OFFSET_FROM_PLAYER: Point = Point { x: 15, y: 10 };
const CAMERA_BORDER: Rect = camera_border();

const fn camera_border() -> Rect {
    let width = i32::abs(MAIN_VIEW_POSITION.x2 - MAIN_VIEW_POSITION.x1);
    let height = i32::abs(MAIN_VIEW_POSITION.y2 - MAIN_VIEW_POSITION.y1);

    Rect {
        x1: MAP_BORDER.x1 + width / 2,
        y1: MAP_BORDER.y1 + height / 2,
        x2: MAP_BORDER.x2 - (width - width / 2),
        y2: MAP_BORDER.y2 - (height - height / 2),
    }
}
pub struct Camera {
    position: Point,
}

impl Camera {
    pub fn new(position: Point) -> Self {
        Camera { position }
    }

    pub fn _get_position(&self) -> &Point {
        &self.position
    }
    pub fn get_start_view(&self) -> Point {
        self.position - MAIN_VIEW_POSITION.center()
    }
}

pub fn move_camera(gs: &State) {
    let pos = gs.world.view::<&Point>();
    let pos = pos.try_get(gs.player_id).expect("Player not exists");
    let mut camera = gs
        .world
        .get_resource_mut::<Camera>()
        .expect("Camera resource not exists");
    camera.position.x = min(
        pos.x + OFFSET_FROM_PLAYER.x,
        max(pos.x - OFFSET_FROM_PLAYER.x, camera.position.x),
    );

    camera.position.y = min(
        pos.y + OFFSET_FROM_PLAYER.y,
        max(pos.y - OFFSET_FROM_PLAYER.y, camera.position.y),
    );

    camera.position.x = min(CAMERA_BORDER.x2, max(CAMERA_BORDER.x1, camera.position.x));
    camera.position.y = min(CAMERA_BORDER.y2, max(CAMERA_BORDER.y1, camera.position.y));
}

pub fn render_map(gs: &State, draw_batch: &mut DrawBatch) {
    let map_field = gs.world.get_resource::<MapField>().unwrap();
    let start_view_correction =
        gs.get_camera_start_view() - Point::new(MAP_BORDER.x1, MAP_BORDER.y1);

    for p in MAIN_VIEW_POSITION.point_set() {
        let (m, n) = (p + start_view_correction).to_unsigned_tuple();
        let (sym, color) = match map_field[m][n] {
            MapTile::Grass => (
                'ш',
                rltk::ColorPair::new(rltk::LIGHT_GREEN, rltk::SANDY_BROWN),
            ),
            MapTile::Ground => (
                '_',
                rltk::ColorPair::new(rltk::SANDY_BROWN, rltk::SANDY_BROWN),
            ),
            MapTile::Water => ('~', rltk::ColorPair::new(rltk::ALICEBLUE, rltk::BLUE)),
        };
        draw_batch.set(p, color, sym);
    }
    draw_batch.submit(MAP_Z_ORDER).expect("Batch error");
}
pub fn render_dynamic(gs: &State, draw_batch: &mut DrawBatch) {
    let start_view = gs.get_camera_start_view();

    gs.world
        .view::<(&Point, &Renderable)>()
        .iter()
        .filter(|(_, ren)| ren.is_visible)
        .map(|(pos, ren)| (*pos - start_view, ren))
        .filter(|(pos, _)| MAIN_VIEW_POSITION.point_in_rect(*pos))
        .for_each(|(pos, ren)| {
            draw_batch.set(pos, ren.color, ren.glyph);
        });
    draw_batch.submit(CHAR_Z_ORDER).expect("Batch error");
}

impl State {
    pub fn get_camera_start_view(self: &State) -> Point {
        self.world
            .get_resource::<Camera>()
            .expect("Camera resource not exists")
            .get_start_view()
    }
}
