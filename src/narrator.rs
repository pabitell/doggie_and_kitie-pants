use pabitell_lib::{data, Event, Narrator, World};
use serde_json::Value;

use crate::events::{self, ProtocolEvent};

#[derive(Default, Debug)]
pub struct Pants;

impl Narrator for Pants {
    fn all_events(&self, _world: &dyn World) -> Vec<Box<dyn Event>> {
        let mut res: Vec<Box<dyn Event>> = vec![];

        // Talk at home
        for (character, idx) in &[
            ("doggie", 0),
            ("kitie", 1),
            ("doggie", 2),
            ("kitie", 3),
            ("doggie", 4),
        ] {
            res.push(Box::new(events::make_talk(
                "talk_at_home",
                data::TalkData::new(character, "home", *idx),
            )));
        }

        // Move to bushes
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move(
                "move_to_bushes",
                data::MoveData::new(character, "bushes"),
                "home",
                Some(5),
                None,
                false,
            )));
        }

        // Talk in bushes
        for (character, idx) in &[("kitie", 0), ("doggie", 2), ("kitie", 3)] {
            res.push(Box::new(events::make_talk(
                "talk_in_bushes",
                data::TalkData::new(character, "bushes", *idx),
            )));
        }

        // Find the rabbit
        res.push(Box::new(events::make_find(
            "rabbit",
            data::UseItemData::new("doggie", "rabbit"),
            "bushes",
            1,
        )));

        // Move to meadow
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move(
                "move_to_meadow",
                data::MoveData::new(character, "meadow"),
                "bushes",
                Some(4),
                None,
                false,
            )));
        }

        // Talk on meadow
        for (character, idx) in &[
            ("doggie", 0),
            ("kitie", 1),
            ("doggie", 2),
            ("kitie", 3),
            ("doggie", 4),
            ("kitie", 5),
            ("doggie", 6),
        ] {
            res.push(Box::new(events::make_talk(
                "talk_on_meadow",
                data::TalkData::new(character, "meadow", *idx),
            )));
        }

        // Pick the earthworm
        res.push(Box::new(events::make_pick(
            "pick_earthworm",
            data::PickData::new("kitie", "earthworm"),
            "meadow".to_string(),
            Some(7),
            false,
        )));
        // Use earthworm
        res.push(Box::new(events::make_use_inventory(
            "use_earthworm",
            data::UseItemData::new("kitie", "earthworm"),
            "meadow",
            Some(7),
            true,
        )));

        // Talk on meadow
        for (character, idx) in &[("doggie", 8)] {
            res.push(Box::new(events::make_talk(
                "talk_on_meadow",
                data::TalkData::new(character, "meadow", *idx),
            )));
        }

        // Move to courtyard
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move(
                "move_to_courtyard",
                data::MoveData::new(character, "courtyard"),
                "meadow",
                Some(9),
                None,
                false,
            )));
        }

        // Talk on courtyard
        for (character, idx) in &[("kitie", 0)] {
            res.push(Box::new(events::make_talk(
                "talk_in_courtyard",
                data::TalkData::new(character, "courtyard", *idx),
            )));
        }

        // Move to dressmakers
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move(
                "move_to_dressmakers",
                data::MoveData::new(character, "dressmakers"),
                "courtyard",
                Some(1),
                None,
                false,
            )));
        }

        // Talk at dressmakers
        for (character, idx) in &[("kitie", 0), ("doggie", 1), ("kitie", 2)] {
            res.push(Box::new(events::make_talk(
                "talk_at_dressmakers",
                data::TalkData::new(character, "dressmakers", *idx),
            )));
        }

        // Find mice
        res.push(Box::new(events::make_find(
            "mice",
            data::UseItemData::new("kitie", "mice"),
            "dressmakers",
            3,
        )));

        res
    }

    fn parse_event(&self, _world: &dyn World, value: Value) -> Option<Box<dyn Event>> {
        let event: Result<ProtocolEvent, serde_json::Error> = serde_json::from_value(value);

        match event {
            Ok(ProtocolEvent::TalkAtHome(data)) => {
                Some(Box::new(events::make_talk("talk_at_home", data)))
            }
            Ok(ProtocolEvent::MoveToBushes(data)) => Some(Box::new(events::make_move(
                "move_to_bushes",
                data,
                "home",
                Some(5),
                None,
                false,
            ))),
            Ok(ProtocolEvent::TalkInBushes(data)) => {
                Some(Box::new(events::make_talk("talk_in_bushes", data)))
            }
            Ok(ProtocolEvent::FindRabbit(data)) => {
                Some(Box::new(events::make_find("rabbit", data, "bushes", 1)))
            }
            Ok(ProtocolEvent::MoveToMeadow(data)) => Some(Box::new(events::make_move(
                "move_to_meadow",
                data,
                "bushes",
                Some(4),
                None,
                false,
            ))),
            Ok(ProtocolEvent::TalkOnMeadow(data)) => {
                Some(Box::new(events::make_talk("talk_on_meadow", data)))
            }
            Ok(ProtocolEvent::PickEarthworm(data)) => Some(Box::new(events::make_pick(
                "pick_earthworm",
                data,
                "meadow".to_owned(),
                Some(7),
                false,
            ))),
            Ok(ProtocolEvent::UseEarthworm(data)) => Some(Box::new(events::make_use_inventory(
                "use_earthworm",
                data,
                "meadow",
                Some(7),
                true,
            ))),
            Ok(ProtocolEvent::MoveToCourtyard(data)) => Some(Box::new(events::make_move(
                "move_to_courtyard",
                data,
                "meadow",
                Some(9),
                None,
                false,
            ))),
            Ok(ProtocolEvent::TalkInCourtyard(data)) => {
                Some(Box::new(events::make_talk("talk_in_courtyard", data)))
            }
            Ok(ProtocolEvent::MoveToDressmakers(data)) => Some(Box::new(events::make_move(
                "move_to_dressmakers",
                data,
                "courtyard",
                Some(1),
                None,
                false,
            ))),
            Ok(ProtocolEvent::TalkAtDressmakers(data)) => {
                Some(Box::new(events::make_talk("talk_at_dressmakers", data)))
            }
            Ok(ProtocolEvent::FindMice(data)) => {
                Some(Box::new(events::make_find("mice", data, "dressmakers", 3)))
            }
            Err(_) => None,
        }
    }
}
