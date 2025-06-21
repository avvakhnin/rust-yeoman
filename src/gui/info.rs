use rltk::{DrawBatch, Point};

use crate::{State, terminal_constants::INFO_Z_ORDER};

use super::layout::STATIC_GUI;

pub fn draw_info(gs: &State, draw_batch: &mut DrawBatch) {
    let pos = gs.world.view::<&Point>();
    let pos = pos.try_get(gs.player_id).expect("Player does not exist");

    let pos = format!("{} {} ", pos.x, pos.y);
    draw_batch.print(Point::new(1, STATIC_GUI.y1 + 1), pos);
    draw_batch.submit(INFO_Z_ORDER).expect("Batch error");
}
