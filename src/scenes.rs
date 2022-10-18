use anyhow::anyhow;
use pabitell_lib::{scene_no_music, scene_with_dialog, AsAny, Music, Named, World};
use serde_json::Value;
use std::any::Any;

scene_with_dialog!(Home, "home", []);
scene_no_music!(Home);

scene_with_dialog!(Bushes, "bushes", []);
scene_no_music!(Bushes);

scene_with_dialog!(Meadow, "meadow", []);
scene_no_music!(Meadow);

scene_with_dialog!(Courtyard, "courtyard", []);
scene_no_music!(Courtyard);

scene_with_dialog!(Dressmakers, "dressmakers", []);

impl Music for Dressmakers {
    fn music(&self) -> Option<String> {
        if self.dialog > 1 {
            Some("music/tancuj-tancuj-vykrucaj.ogg".to_owned())
        } else {
            None
        }
    }
}
