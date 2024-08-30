use std::fs::read_to_string;

use anathema::prelude::*;
use anathema::component::*;
use anathema::widgets::components::events::KeyState;

#[derive(State)]
struct PlayerState {
    pos_x: Value<i64>,
    pos_y: Value<i64>,
}

struct Player;

impl Component for Player {
    type Message = ();
    type State = PlayerState;

    fn on_key(
        &mut self,
        key: KeyEvent,
        state: &mut Self::State,
        elements: Elements<'_, '_>,
        context: Context<'_, Self::State>,
    ) {
        let mut pos_x = state.pos_x.to_mut();
        let mut pos_y = state.pos_y.to_mut();
        
        if matches!(key.state, KeyState::Press) {
            match key.get_char() {
                Some('w') => *pos_y -= 1,
                Some('a') => *pos_x -= 1,
                Some('s') => *pos_y += 1,
                Some('d') => *pos_x += 1,
                _ => ()
            }
        }
    }
}

fn main() {
    let main_template = read_to_string("templates/main.aml").unwrap();

    let doc = Document::new(main_template);

    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .hide_cursor()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);

    let _ = runtime.register_component(
        "player",
        "templates/player.aml",
        Player, 
        PlayerState {
            pos_x: 24.into(),
            pos_y: 24.into()
        }
    );

    runtime.finish().unwrap().run();
}
