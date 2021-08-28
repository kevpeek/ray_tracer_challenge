use crate::tracing::world::World;
use crate::exercises::snapshot;
use crate::display::resolution::Resolution;

mod display;
mod exercises;
mod geometry;
mod helper;
mod tracing;

fn main() {
    let world = exercises::world_one::make_world();
    let camera = snapshot::make_camera_one(Resolution::LOW);
    exercises::snapshot::snapshot_world(world, camera)
}

