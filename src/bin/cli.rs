use pabitell_lib::{cli::cmdline, WorldBuilder};

pub use common::items;
pub use common::narrator;
pub use common::world;

pub fn main() {
    let world = world::PantsWorldBuilder::make_world().unwrap();
    let narrator = narrator::Pants;
    cmdline::run("doggie_and_kitie-pants", world, narrator);
}
