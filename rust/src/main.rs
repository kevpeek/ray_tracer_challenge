use crate::tracing::world::World;

mod display;
mod exercises;
mod geometry;
mod helper;
mod tracing;

fn main() {
    let world = exercises::worlds::world_one();
    run_world(world);
}

fn run_world(world: World) {
    exercises::worlds::run_world(world)
}
