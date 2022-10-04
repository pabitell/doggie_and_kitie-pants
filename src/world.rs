use anyhow::{anyhow, Result};
use pabitell_lib::{
    translations::get_available_locales, Character, Clean, Description, Dumpable, Item, ItemState,
    Named, Scene, Tagged, World, WorldBuilder,
};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    characters, items, scenes,
    translations::{get_message, RESOURCES},
};

const DEFAULT_LANG: &str = "cs";

#[derive(Debug, Default)]
pub struct PantsWorld {
    id: Uuid,
    lang: String,
    items: HashMap<String, Box<dyn Item>>,
    characters: HashMap<String, Box<dyn Character>>,
    scenes: HashMap<String, Box<dyn Scene>>,
    event_count: usize,
}

struct PantsWorldDescription;
impl Named for PantsWorldDescription {
    fn name(&self) -> &'static str {
        "description"
    }
}

impl Description for PantsWorldDescription {
    fn long(&self, world: &dyn World) -> String {
        world.get_message(&format!("{}-long", world.name()), None)
    }

    fn short(&self, world: &dyn World) -> String {
        world.get_message(&format!("{}-short", world.name()), None)
    }
}

#[derive(Default)]
pub struct PantsWorldBuilder {
    items: Vec<Box<dyn Item>>,
    characters: Vec<Box<dyn Character>>,
    scenes: Vec<Box<dyn Scene>>,
}

impl WorldBuilder<PantsWorld> for PantsWorldBuilder {
    fn character(mut self, character: Box<dyn Character>) -> Self {
        self.characters.push(character);
        self
    }

    fn item(mut self, item: Box<dyn Item>) -> Self {
        self.items.push(item);
        self
    }

    fn scene(mut self, scene: Box<dyn Scene>) -> Self {
        self.scenes.push(scene);
        self
    }

    fn build(self) -> Result<PantsWorld> {
        Ok(PantsWorld {
            lang: DEFAULT_LANG.into(),
            characters: self
                .characters
                .into_iter()
                .map(|e| (e.name().into(), e))
                .collect(),
            items: self
                .items
                .into_iter()
                .map(|e| (e.name().into(), e))
                .collect(),
            scenes: self
                .scenes
                .into_iter()
                .map(|e| (e.name().into(), e))
                .collect(),

            ..Default::default()
        })
    }

    fn make_world() -> Result<PantsWorld> {
        Self::default()
            .scene(Box::new(scenes::Home::default()))
            .scene(Box::new(scenes::Bushes::default()))
            .scene(Box::new(scenes::Meadow::default()))
            .scene(Box::new(scenes::Courtyard::default()))
            .scene(Box::new(scenes::Dressmakers::default()))
            .character(Box::new(characters::Kitie::default()))
            .character(Box::new(characters::Doggie::default()))
            .item(Box::new(items::Rabbit::default()))
            .item(Box::new(items::Earthworm::default()))
            .item(Box::new(items::Mice::default()))
            .build()
    }
}

impl Tagged for PantsWorld {}

impl Named for PantsWorld {
    fn name(&self) -> &'static str {
        "doggie_and_kitie-pants"
    }
}

impl World for PantsWorld {
    fn characters(&self) -> &HashMap<String, Box<dyn Character>> {
        &self.characters
    }

    fn characters_mut(&mut self) -> &mut HashMap<String, Box<dyn Character>> {
        &mut self.characters
    }

    fn scenes(&self) -> &HashMap<String, Box<dyn Scene>> {
        &self.scenes
    }

    fn scenes_mut(&mut self) -> &mut HashMap<String, Box<dyn Scene>> {
        &mut self.scenes
    }

    fn items(&self) -> &HashMap<String, Box<dyn Item>> {
        &self.items
    }

    fn items_mut(&mut self) -> &mut HashMap<String, Box<dyn Item>> {
        &mut self.items
    }

    fn description(&self) -> Box<dyn Description> {
        Box::new(PantsWorldDescription)
    }

    fn lang(&self) -> &str {
        &self.lang
    }

    fn set_lang(&mut self, lang: &str) -> bool {
        if let Ok(locales) = get_available_locales(&RESOURCES) {
            if locales.iter().any(|l| *l == lang) {
                self.lang = lang.into();
                return true;
            }
        }
        false
    }

    fn available_languages(&self) -> Vec<String> {
        get_available_locales(&RESOURCES)
            .unwrap_or_else(|_| vec![])
            .iter()
            .map(|e| e.to_string())
            .collect()
    }

    fn setup(&mut self, new_id: bool) {
        if new_id {
            self.randomize_id();
        }

        self.items_mut().values_mut().for_each(|i| {
            i.set_state(match i.name() {
                "rabbit" => ItemState::InScene("bushes".to_owned()),
                "earthworm" => ItemState::InScene("meadow".to_owned()),
                "mice" => ItemState::InScene("dressmakers".to_owned()),
                _ => ItemState::Unassigned,
            })
        });
    }

    fn finished(&self) -> bool {
        self.scenes().get("dressmakers").unwrap().dialog() == Some(4)
    }

    fn event_count(&self) -> usize {
        self.event_count
    }

    fn event_inc(&mut self) {
        self.event_count += 1;
    }

    fn id(&self) -> &Uuid {
        &self.id
    }
    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }
    fn get_message(&self, msgid: &str, args: Option<fluent::FluentArgs>) -> String {
        get_message(msgid, &self.lang, args)
    }
    fn version(&self) -> usize {
        1
    }
}

impl Clean for PantsWorld {
    fn clean(&mut self) {
        self.event_count = 0;
    }
}

impl Dumpable for PantsWorld {
    fn dump(&self) -> serde_json::Value {
        serde_json::json!({
            "characters": self.characters.iter().map(|(k, v)| (k.clone(), v.dump())).collect::<HashMap<String, serde_json::Value>>(),
            "items": self.items.iter().map(|(k, v)| (k.clone(), v.dump())).collect::<HashMap<String, serde_json::Value>>(),
            "scenes": self.scenes.iter().map(|(k, v)| (k.clone(), v.dump())).collect::<HashMap<String, serde_json::Value>>(),
            "event_count": self.event_count,
        })
    }
    fn load(&mut self, data: serde_json::Value) -> Result<()> {
        match data {
            // TODO it might be required to check whether all characters, items and scenes exist
            // before loading data
            serde_json::Value::Object(root) => {
                for item in root {
                    match item {
                        (k, serde_json::Value::Object(characters)) if k == "characters" => {
                            for (name, data) in characters.into_iter() {
                                let character = self
                                    .characters_mut()
                                    .get_mut(&name)
                                    .ok_or_else(|| anyhow!("missing character '{}'", name))?;
                                character.load(data)?;
                            }
                        }
                        (k, serde_json::Value::Object(items)) if k == "items" => {
                            for (name, data) in items.into_iter() {
                                let item = self
                                    .items_mut()
                                    .get_mut(&name)
                                    .ok_or_else(|| anyhow!("missing item '{}'", name))?;
                                item.load(data)?;
                            }
                        }
                        (k, serde_json::Value::Object(scenes)) if k == "scenes" => {
                            for (name, data) in scenes.into_iter() {
                                let scene = self
                                    .scenes_mut()
                                    .get_mut(&name)
                                    .ok_or_else(|| anyhow!("missing scene '{}'", name))?;
                                scene.load(data)?;
                            }
                        }
                        (k, serde_json::Value::Number(num)) if k == "event_count" => {
                            if let Some(count) = num.as_u64() {
                                self.event_count = count as usize;
                            } else {
                                return Err(anyhow!(""));
                            }
                        }
                        _ => return Err(anyhow!("")),
                    }
                }
            }
            _ => return Err(anyhow!("")),
        }
        Ok(())
    }
}
