use crate::display::resolution::Resolution;
use crate::exercises::snapshot;

mod display;
mod exercises;
mod geometry;
mod tracing;
mod helpers;

fn main() {
    let world = exercises::sandbox::make_world();
    let camera = snapshot::make_camera_one(Resolution::LOW);
    exercises::snapshot::snapshot_world(world, camera)
}
