use edict::{
    entity::EntityId,
    flow::FlowWorld,
    prelude::{ChildOf, Relation},
    world::World,
};
use rltk::{ColorPair, Point, RGBA};

use crate::components::{PlanJob, Renderable, rotate_render_stack};

pub fn create_plan_job(world: &mut World, pos: Point, plan_owner: EntityId) {
    let plan_job = world
        .spawn_external((
            pos,
            Renderable::new_bg('T', rltk::BLUE3, rltk::CADET_BLUE),
            PlanJob {},
        ))
        .id();
    world.insert_relation(plan_job, ChildOf, plan_owner);
}

pub fn create_plant_flow(world: &mut FlowWorld, pos: Point) {
    world
        .spawn_external((pos, Renderable::new_blank()))
        .spawn_flow(rotate_render_stack);
}
