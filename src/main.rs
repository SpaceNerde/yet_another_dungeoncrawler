use std::fs::read_to_string;

use anathema::prelude::*;
use anathema::component::*;

#[derive(State)]
struct PlayerPosition {
    pos_x: Value<i64>,
    pos_y: Value<i64>,
}

struct Player;

impl Component for Player {
    type Message = ();
    type State = PlayerPosition;
}

fn main() {
    let main_template = read_to_string("templates/main.aml").unwrap();

    let doc = Document::new(main_template);

    let mut backend = TuiBackend::builder()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);

    let comp_id = runtime.register_component(
        "player",
        "templates/player.aml",
        Player, 
        PlayerPosition {
            pos_x: 24.into(),
            pos_y: 24.into()
        }
    );

    runtime.finish().unwrap().run();
}
