use crate::world::PantsWorldBuilder;
use pabitell_lib::{World, WorldBuilder};

pub fn make_world(lang: &str) -> Box<dyn World> {
    let mut world = PantsWorldBuilder::make_world().unwrap();
    world.setup(true);
    world.set_lang(lang);
    Box::new(world)
}
