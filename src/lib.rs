pub mod characters;
pub mod events;
pub mod items;
pub mod narrator;
pub mod scenes;
pub mod translations;
#[cfg(feature = "with_webapp")]
pub mod webapp;
pub mod world;

#[cfg(test)]
pub mod tests {
    use pabitell_lib::{Dumpable, Event, ItemState, Narrator, World, WorldBuilder};

    use crate::{
        narrator,
        world::{PantsWorld, PantsWorldBuilder},
    };

    pub fn prepare_world() -> PantsWorld {
        let mut world = PantsWorldBuilder::make_world().unwrap();
        world.setup(true);
        world
    }

    #[test]
    fn setup() {
        let world = prepare_world();
        assert_eq!(world.characters().get("kitie").unwrap().scene(), &None);
        assert_eq!(world.characters().get("doggie").unwrap().scene(), &None);
        assert_eq!(
            world.items().get("rabbit").unwrap().state(),
            &ItemState::InScene("bushes".into())
        );
        assert_eq!(
            world.items().get("earthworm").unwrap().state(),
            &ItemState::InScene("meadow".into())
        );
        assert_eq!(
            world.items().get("mice").unwrap().state(),
            &ItemState::InScene("dressmakers".into())
        );
    }

    fn reload_world(world: PantsWorld) -> PantsWorld {
        let mut new_world = PantsWorldBuilder::make_world().unwrap();
        new_world.load(world.dump()).unwrap();
        assert_eq!(new_world.dump(), world.dump());
        new_world
    }

    fn reload_events(
        world: &dyn World,
        narrator: &dyn Narrator,
        events: Vec<Box<dyn Event>>,
    ) -> Vec<Box<dyn Event>> {
        assert!(events.iter().all(|e| e.can_be_triggered(world)));
        let res = events
            .iter()
            .map(|e| {
                narrator
                    .parse_event(world, e.dump())
                    .ok_or_else(|| anyhow::anyhow!("parse_failed"))
            })
            .collect::<Result<Vec<Box<dyn Event>>, _>>()
            .unwrap();
        assert!(res.iter().all(|e| e.can_be_triggered(world)));
        res
    }

    #[test]
    fn workflow() {
        let mut world = prepare_world();
        world = reload_world(world);

        let narrator = narrator::Pants::default();

        // move characters home
        let events = narrator.available_events(&world);
        let events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 2);
        for mut event in events {
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
        }

        // talk at home
        for _ in 0..5 {
            let events = narrator.available_events(&world);
            let mut events = reload_events(&world, &narrator, events);
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].name(), "talk_at_home");
            assert!(events[0].can_be_triggered(&world));
            assert!(events[0].perform(&mut world));
        }

        // move characters to bushes
        let events = narrator.available_events(&world);
        let events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 2);
        for mut event in events {
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
        }

        // Talk in bushes
        for _ in 0..1 {
            let events = narrator.available_events(&world);
            let mut events = reload_events(&world, &narrator, events);
            assert_eq!(events.len(), 1);
            assert!(events[0].can_be_triggered(&world));
            assert!(events[0].perform(&mut world));
        }

        // Find rabbit
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 1);
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));

        // Talk on in bushes
        for _ in 0..2 {
            let events = narrator.available_events(&world);
            let mut events = reload_events(&world, &narrator, events);
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].name(), "talk_in_bushes");
            assert!(events[0].can_be_triggered(&world));
            assert!(events[0].perform(&mut world));
        }

        // Move to meadow
        let events = narrator.available_events(&world);
        let events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 2);
        for mut event in events {
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
        }

        // Talk on meadow
        for _ in 0..7 {
            let events = narrator.available_events(&world);
            let mut events = reload_events(&world, &narrator, events);
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].name(), "talk_on_meadow");
            assert!(events[0].can_be_triggered(&world));
            assert!(events[0].perform(&mut world));
        }

        // Pick earthworm
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name(), "pick_earthworm");
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));

        // Use earthworm
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name(), "use_earthworm");
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));

        // Talk on meadow
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name(), "talk_on_meadow");
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));

        // Move to courtyard
        let events = narrator.available_events(&world);
        let events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 2);
        for mut event in events {
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
        }

        // Talk on courtyard
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name(), "talk_in_courtyard");
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));

        // Move to dressmakers
        let events = narrator.available_events(&world);
        let events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 2);
        for mut event in events {
            assert!(event.can_be_triggered(&world));
            assert!(event.perform(&mut world));
        }

        // Talk at dressmakers
        for _ in 0..3 {
            let events = narrator.available_events(&world);
            let mut events = reload_events(&world, &narrator, events);
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].name(), "talk_at_dressmakers");
            assert!(events[0].can_be_triggered(&world));
            assert!(events[0].perform(&mut world));
        }

        // Catch mice
        let events = narrator.available_events(&world);
        let mut events = reload_events(&world, &narrator, events);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name(), "find_mice");
        assert!(events[0].can_be_triggered(&world));
        assert!(events[0].perform(&mut world));

        assert!(world.finished());
    }

    #[test]
    fn languages() {
        let mut world = PantsWorldBuilder::make_world().unwrap();
        for lang in vec!["cs"] {
            assert!(world.set_lang(lang));
        }
    }
}
