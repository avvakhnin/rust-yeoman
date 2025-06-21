use rltk::{BLACK, BLUE, ColorPair, DrawBatch, GREEN, Point, RGB, RGBA, Rect};

use crate::{State, math::QuasiRect, terminal_constants::CHAR_Z_ORDER};

use super::layout::MAIN_VIEW_POSITION;

pub fn draw_hud(gs: &State, draw_batch: &mut DrawBatch) {
    //_draw_cross(gs, draw_batch);
    render_cursor_field(gs, draw_batch);
}

fn _draw_cross(gs: &State, draw_batch: &mut DrawBatch) {
    let start_view = gs.get_camera_start_view();
    let center = MAIN_VIEW_POSITION.center(); //TODO make constant
    let color = ColorPair::new(RGB::named(GREEN), RGB::named(BLACK));

    for i in MAIN_VIEW_POSITION.x1..MAIN_VIEW_POSITION.x2 {
        draw_batch.set(
            Point::new(i, center.y),
            color,
            (i + start_view.x).to_string().chars().last().unwrap(),
        );
    }
    for i in MAIN_VIEW_POSITION.y1..MAIN_VIEW_POSITION.y2 {
        draw_batch.set(
            Point::new(center.x, i),
            color,
            (i + start_view.y).to_string().chars().last().unwrap(),
        );
    }
    draw_batch.submit(CHAR_Z_ORDER - 1).expect("Batch error");
}

fn render_cursor_field(gs: &State, draw_batch: &mut DrawBatch) {
    let start_view = gs.get_camera_start_view();
    let rect = gs.world.view::<&Rect>();

    let rect = rect.try_get(gs.cursor_id).expect("Cursor does not exist");
    rect.envelop_rect().for_each(|p| {
        draw_batch.set_bg(p - start_view, RGBA::named(BLUE));
    });
    draw_batch.submit(CHAR_Z_ORDER - 3).expect("Batch error");
}
