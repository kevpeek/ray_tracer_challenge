use crate::display::resolution::Resolution;
use crate::exercises::snapshot;

mod display;
mod exercises;
mod geometry;
mod helper;
mod tracing;

fn main() {
    let (world, camera) = exercises::chess::make_world();
    // let camera = snapshot::make_camera_one(Resolution::LOW);
    exercises::snapshot::snapshot_world(world, camera)
}
