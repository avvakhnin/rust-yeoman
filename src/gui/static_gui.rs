use crate::Consoles;

use super::layout::STATIC_GUI;

pub fn draw_static(context: &mut rltk::Rltk) {
    let console = Consoles::AdditionalVga;
    context.set_active_console(console.num());
    context.draw_box_double(
        STATIC_GUI.x1,
        STATIC_GUI.y1,
        STATIC_GUI.width() - 1,
        STATIC_GUI.height() - 1,
        rltk::RGB::named(rltk::RED2),
        rltk::RGB::named(rltk::ALICE_BLUE),
    );
}
