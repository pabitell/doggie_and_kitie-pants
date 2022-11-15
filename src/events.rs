use pabitell_lib::{
    conditions::{
        AllItemsWithTagInStateCheck, CharacterInSceneCheck, Condition, HasItemCheck,
        SameSceneCheck, SceneDialogCheck,
    },
    data, events, updates, Event, ItemState, Tagged,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum ProtocolEvent {
    MoveHome(data::MoveData),
    TalkAtHome(data::TalkData),
    MoveToBushes(data::MoveData),
    FixEar(data::UseItemData),
    TalkInBushes(data::TalkData),
    FindRabbit(data::UseItemData),
    MoveToMeadow(data::MoveData),
    TalkOnMeadow(data::TalkData),
    PickEarthworm(data::PickData),
    UseEarthworm(data::UseItemData),
    MoveToCourtyard(data::MoveData),
    TalkInCourtyard(data::TalkData),
    MoveToDressmakers(data::MoveData),
    TalkAtDressmakers(data::TalkData),
    FindMice(data::UseItemData),
}

fn doggie_and_kitie_in_same_scene() -> Condition {
    SameSceneCheck::cond(vec!["doggie".to_string(), "kitie".to_string()], vec![])
}

pub fn make_talk(name: &str, data: data::TalkData) -> events::Talk {
    let mut event = events::Talk::new(name, data);

    event.set_tags(vec!["talk".to_string(), "no_read".to_string()]);

    event.set_condition(
        CharacterInSceneCheck::cond(event.character().to_owned(), Some(event.scene().to_owned()))
            & SceneDialogCheck::cond(event.scene().to_owned(), event.dialog())
            & doggie_and_kitie_in_same_scene(),
    );

    event.set_world_updates(vec![Box::new(updates::NextSceneDialogChange::new(
        event.scene().to_owned(),
    ))]);

    event
}

pub fn make_move(
    name: &str,
    data: data::MoveData,
    from_scene: Option<String>,
    from_dialog: Option<usize>,
    items_state: Option<(Vec<String>, ItemState)>,
    increase_dialog: bool,
) -> events::Move {
    let mut event = events::Move::new(name, data);

    event.set_tags(vec!["move".to_string()]);

    let mut condition =
        CharacterInSceneCheck::cond(event.character().to_owned(), from_scene.clone());

    if let Some(from_dialog) = from_dialog {
        condition = condition & SceneDialogCheck::cond(from_scene.unwrap(), from_dialog);
    }
    if let Some((tags, state)) = items_state.as_ref() {
        condition = condition & AllItemsWithTagInStateCheck::cond(tags.clone(), state.to_owned());
    }

    event.set_condition(condition);

    let mut updates: Vec<Box<dyn updates::Change>> =
        vec![Box::new(updates::MoveCharacterChange::new(
            event.character().to_owned(),
            Some(event.scene().to_owned()),
        ))];
    if increase_dialog {
        updates.push(Box::new(updates::NextSceneDialogChange::new(
            event.scene().to_owned(),
        )));
    }
    event.set_world_updates(updates);

    event
}

pub fn make_find(
    name: &str,
    data: data::UseItemData,
    scene: &str,
    dialog_idx: usize,
    self_trigger: bool,
) -> events::UseItem {
    let item = data.item.to_string();
    let mut event = events::UseItem::new(name, data);
    let scene = scene.to_string();

    if self_trigger {
        event.set_tags(vec!["find".to_string(), "self-trigger".to_string()]);
    } else {
        event.set_tags(vec!["find".to_string()]);
    }

    event.set_condition(
        SameSceneCheck::cond(
            vec!["doggie".to_owned(), "kitie".to_owned()],
            vec![item.to_owned()],
        ) & SceneDialogCheck::cond(scene.clone(), dialog_idx),
    );

    event.set_world_updates(vec![
        Box::new(updates::AssignItemChange::new(item, ItemState::Unassigned)),
        Box::new(updates::NextSceneDialogChange::new(scene)),
    ]);

    event
}

pub fn make_pick(
    name: &str,
    pick_data: data::PickData,
    scene: String,
    dialog_idx: Option<usize>,
    dialog_inc: bool,
) -> events::Pick {
    let mut event = events::Pick::new(name, pick_data);

    event.set_tags(vec!["pick".to_string()]);

    let mut updates: Vec<Box<dyn updates::Change>> =
        vec![Box::new(updates::AssignItemChange::new(
            event.item().to_owned(),
            ItemState::Owned(event.character().to_owned()),
        ))];
    if dialog_inc {
        updates.push(Box::new(updates::NextSceneDialogChange::new(scene.clone())));
    }
    event.set_world_updates(updates);

    let mut condition =
        SameSceneCheck::cond(
            vec![event.character().to_string()],
            vec![event.item().to_string()],
        ) & CharacterInSceneCheck::cond(event.character().to_owned(), Some(scene.to_owned()));
    if let Some(dialog_idx) = dialog_idx {
        condition = condition & SceneDialogCheck::cond(scene, dialog_idx);
    }
    event.set_condition(condition);

    event
}

pub fn make_use_inventory(
    name: &str,
    use_item_data: data::UseItemData,
    scene: &str,
    dialog_idx: Option<usize>,
    dialog_inc: bool,
) -> events::UseItem {
    let mut event = events::UseItem::new(name, use_item_data);
    event.set_tags(vec!["use".to_string()]);

    let mut updates: Vec<Box<dyn updates::Change>> = vec![Box::new(
        updates::AssignItemChange::new(event.item().to_owned(), ItemState::Unassigned),
    )];
    if dialog_inc {
        updates.push(Box::new(updates::NextSceneDialogChange::new(
            scene.to_owned(),
        )));
    }
    event.set_world_updates(updates);

    let mut condition = doggie_and_kitie_in_same_scene()
        & CharacterInSceneCheck::cond(event.character().to_owned(), Some(scene.to_owned()))
        & HasItemCheck::cond(event.character().to_owned(), event.item().to_owned());
    if let Some(dialog_idx) = dialog_idx {
        condition = condition & SceneDialogCheck::cond(scene.to_owned(), dialog_idx);
    }
    event.set_condition(condition);

    event
}
