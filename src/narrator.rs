use pabitell_lib::{data, Event, Narrator, World};
use serde_json::Value;

use crate::events::{self, ProtocolEvent};

#[derive(Default, Debug)]
pub struct Pants;

impl Narrator for Pants {
    fn all_events(&self, _world: &dyn World) -> Vec<Box<dyn Event>> {
        let mut res: Vec<Box<dyn Event>> = vec![];

        // Move home first
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move(
                "move_home",
                data::MoveData::new(character, "home"),
                None,
                None,
                None,
                false,
            )));
        }

        // Talk at home
        for (character, idx) in &[("doggie", 0), ("kitie", 1), ("doggie", 2), ("kitie", 3)] {
            res.push(Box::new(events::make_talk(
                "talk_at_home",
                data::TalkData::new(character, "home", *idx),
            )));
        }

        // Fix doggie's ear
        res.push(Box::new(events::make_find(
            "fix_ear",
            data::UseItemData::new("doggie", "ear"),
            "home",
            4,
            true,
        )));

        // Move to bushes
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move(
                "move_to_bushes",
                data::MoveData::new(character, "bushes"),
                Some("home".to_string()),
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
            "find_rabbit",
            data::UseItemData::new("doggie", "rabbit"),
            "bushes",
            1,
            false,
        )));

        // Move to meadow
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move(
                "move_to_meadow",
                data::MoveData::new(character, "meadow"),
                Some("bushes".to_string()),
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
        res.push(Box::new(events::make_talk(
            "talk_on_meadow",
            data::TalkData::new("doggie", "meadow", 8),
        )));

        // Move to courtyard
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move(
                "move_to_courtyard",
                data::MoveData::new(character, "courtyard"),
                Some("meadow".to_string()),
                Some(9),
                None,
                false,
            )));
        }

        // Talk on courtyard
        res.push(Box::new(events::make_talk(
            "talk_in_courtyard",
            data::TalkData::new("kitie", "courtyard", 0),
        )));

        // Move to dressmakers
        for character in &["doggie", "kitie"] {
            res.push(Box::new(events::make_move(
                "move_to_dressmakers",
                data::MoveData::new(character, "dressmakers"),
                Some("courtyard".to_string()),
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
            "find_mice",
            data::UseItemData::new("kitie", "mice"),
            "dressmakers",
            3,
            false,
        )));

        res
    }

    fn parse_event(&self, _world: &dyn World, value: Value) -> Option<Box<dyn Event>> {
        let event: Result<ProtocolEvent, serde_json::Error> = serde_json::from_value(value);

        match event {
            Ok(ProtocolEvent::TalkAtHome(data)) => {
                Some(Box::new(events::make_talk("talk_at_home", data)))
            }
            Ok(ProtocolEvent::MoveHome(data)) => Some(Box::new(events::make_move(
                "move_home",
                data,
                None,
                None,
                None,
                false,
            ))),
            Ok(ProtocolEvent::FixEar(data)) => Some(Box::new(events::make_find(
                "fix_ear", data, "home", 4, true,
            ))),
            Ok(ProtocolEvent::MoveToBushes(data)) => Some(Box::new(events::make_move(
                "move_to_bushes",
                data,
                Some("home".to_string()),
                Some(5),
                None,
                false,
            ))),
            Ok(ProtocolEvent::TalkInBushes(data)) => {
                Some(Box::new(events::make_talk("talk_in_bushes", data)))
            }
            Ok(ProtocolEvent::FindRabbit(data)) => Some(Box::new(events::make_find(
                "find_rabbit",
                data,
                "bushes",
                1,
                false,
            ))),
            Ok(ProtocolEvent::MoveToMeadow(data)) => Some(Box::new(events::make_move(
                "move_to_meadow",
                data,
                Some("bushes".to_string()),
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
                Some("meadow".to_owned()),
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
                Some("courtyard".to_owned()),
                Some(1),
                None,
                false,
            ))),
            Ok(ProtocolEvent::TalkAtDressmakers(data)) => {
                Some(Box::new(events::make_talk("talk_at_dressmakers", data)))
            }
            Ok(ProtocolEvent::FindMice(data)) => Some(Box::new(events::make_find(
                "find_mice",
                data,
                "dressmakers",
                3,
                false,
            ))),
            Err(_) => None,
        }
    }
}
