mod ambience;
mod camera;
mod components;
mod control;
mod flow_timer;
mod gui;
mod math;
mod terminal_constants;

use camera::{Camera, move_camera};
use components::Renderable;
use control::{ControlMode, player_input};
use edict::{entity::EntityId, flow::Flows, scheduler::Scheduler, world::World};
use flow_timer::init_flow_timers;
use rltk::{DrawBatch, GameState, Point, Rect, Rltk, render_draw_buffer};
use terminal_constants::Consoles;

rltk::embedded_resource!(TTILE_FONT3, "../resources/unicode_16x16.png");

fn main() -> rltk::BError {
    rltk::link_resource!(TTILE_FONT3, "resources/unicode_16x16.png");

    use rltk::RltkBuilder;
    let Point { x: mw, y: mh } = Consoles::Main.dimensions();
    let Point { x: aw, y: ah } = Consoles::AdditionalVga.dimensions();
    let (mfw, mfh) = Consoles::Main.font_dimensions();
    let (afw, afh) = Consoles::AdditionalVga.font_dimensions();
    let mut context = RltkBuilder::new()
        .with_title("Однодворец")
        .with_dimensions(mw, mh)
        .with_tile_dimensions(mfw, mfh)
        .with_font(Consoles::Main.font(), mfw, mfh)
        .with_simple_console(mw, mh, Consoles::Main.font())
        .with_font(Consoles::AdditionalVga.font(), afw, afh)
        .with_simple_console_no_bg(aw, ah, Consoles::AdditionalVga.font())
        .build()?;

    context.set_translation_mode(0, rltk::CharacterTranslationMode::Unicode);

    gui::static_gui::draw_static(&mut context);

    let mut world = World::new();
    let map = ambience::map::random_map();
    let start_position = Point::new(20, 50);
    world.insert_resource(Camera::new(start_position));
    world.insert_resource(map);
    world.insert_resource(DrawBatch::new());
    world.insert_resource(0f32);

    world.ensure_external_registered::<Point>();
    world.ensure_external_registered::<Rect>();
    world.ensure_external_registered::<Renderable>();

    let player_id = world
        .spawn_external((start_position, Renderable::new('Ӂ', rltk::RED)))
        .id();
    let cursor_id = world
        .spawn_external((Rect::with_exact(20, 50, 20, 50),))
        .id();

    let mut scheduler = Scheduler::new();
    init_flow_timers(&mut world, &mut scheduler);
    let gs = State {
        world,
        scheduler,
        flows: Flows::new(),
        mode: ControlMode::Player,
        player_id,
        cursor_id,
    };
    rltk::main_loop(context, gs)
}

struct State {
    pub world: World,
    scheduler: Scheduler,
    flows: Flows,
    pub mode: ControlMode,
    pub player_id: EntityId,
    pub cursor_id: EntityId,
}

impl GameState for State {
    fn tick(&mut self, context: &mut Rltk) {
        self.world.insert_resource::<f32>(context.frame_time_ms);
        player_input(self, context);
        self.flows.execute(&mut self.world);
        self.scheduler.run_sequential(&mut self.world);
        move_camera(self);
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(Consoles::AdditionalVga.num());
        gui::info::draw_info(self, &mut draw_batch);

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(Consoles::Main.num());
        camera::render_map(self, &mut draw_batch);
        camera::render_dynamic(self, &mut draw_batch);
        gui::hud::draw_hud(self, &mut draw_batch);

        render_draw_buffer(context).expect("Render error");
    }
}
