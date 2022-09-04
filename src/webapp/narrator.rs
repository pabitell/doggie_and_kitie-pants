use crate::narrator::Pants;
use pabitell_lib::Narrator;

pub fn make_narrator() -> Box<dyn Narrator> {
    Box::new(Pants::default())
}
