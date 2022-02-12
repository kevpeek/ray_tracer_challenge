use crate::display::resolution::Resolution;

mod display;
mod exercises;
mod geometry;
mod helpers;
mod tracing;

fn main() {
    let (world, camera_maker) = exercises::spheres::make_world();
    let camera = camera_maker(Resolution::FHD);
    exercises::snapshot::snapshot_world(world, camera)
}
