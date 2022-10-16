use pabitell_lib::{data, webapp::print::PrintItem, World};
use std::rc::Rc;

use crate::events::ProtocolEvent;

pub fn make_print_items(world: Box<dyn World>) -> Vec<PrintItem> {
    let mut res = vec![];

    // fix eark
    let item = world.items().get("ear").unwrap();
    let data = serde_json::to_value(ProtocolEvent::FixEar(data::UseItemData::new(
        "doggie", "ear",
    )))
    .unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(item.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", item.name()))),
    );

    // move to bushes
    let scene = world.scenes().get("bushes").unwrap();
    let data = serde_json::to_value(ProtocolEvent::MoveToBushes(data::MoveData::new(
        "", "bushes",
    )))
    .unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // find rabbit
    let item = world.items().get("rabbit").unwrap();
    let data = serde_json::to_value(ProtocolEvent::FindRabbit(data::UseItemData::new(
        "", "doll",
    )))
    .unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(item.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", item.name()))),
    );

    // move to meadow
    let scene = world.scenes().get("meadow").unwrap();
    let data = serde_json::to_value(ProtocolEvent::MoveToMeadow(data::MoveData::new(
        "", "meadow",
    )))
    .unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // pick earthworm
    let item = world.items().get("earthworm").unwrap();
    let data = serde_json::to_value(ProtocolEvent::PickEarthworm(data::PickData::new(
        "",
        "earthworm",
    )))
    .unwrap();

    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(item.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", item.name()))),
    );

    // move to courtyard
    let scene = world.scenes().get("courtyard").unwrap();
    let data = serde_json::to_value(ProtocolEvent::MoveToCourtyard(data::MoveData::new(
        "",
        "courtyard",
    )))
    .unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // move to dressmakers
    let scene = world.scenes().get("dressmakers").unwrap();
    let data = serde_json::to_value(ProtocolEvent::MoveToDressmakers(data::MoveData::new(
        "",
        "dressmakers",
    )))
    .unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(scene.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", scene.name()))),
    );

    // catch mice
    let item = world.items().get("mice").unwrap();
    let data =
        serde_json::to_value(ProtocolEvent::FindMice(data::UseItemData::new("", "mice"))).unwrap();
    res.push(
        PrintItem::new(Rc::new(data.to_string().as_bytes().to_vec()))
            .title(Some(item.short(world.as_ref())))
            .img_url(Some(format!("images/{}.svg", item.name()))),
    );

    res
}
