use crate::display::resolution::Resolution;
use crate::exercises::snapshot;

mod display;
mod exercises;
mod geometry;
mod helpers;
mod tracing;

fn main() {
    let (world, camera_maker) = exercises::chess::make_world();
    let camera = camera_maker(Resolution::FHD);
    exercises::snapshot::snapshot_world(world, camera)
}
