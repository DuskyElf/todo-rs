use crate::{self as lib, CoreState, CuiState};

pub fn start() {
    let mut core_state = CoreState {
        todo_list: vec![
            "Make a cup of tea".to_owned(),
            "Buy some bread".to_owned(),
            "Improve todo-rs project".to_owned(),
        ],
        done_list: vec![
            "Stay happy".to_owned(),
        ],
    };

    main_loop(&mut core_state);
}

fn main_loop(core_state: &mut CoreState) {
    let mut key_input = None;
    let mut cui_state = CuiState::init(core_state);

    loop {
        let responce = cui_state.update(key_input);
        match responce {
            lib::CuiResponse::Quit => break,
            lib::CuiResponse::UserInput(key) => {
                key_input = key;
            }
        }
    }

    cui_state.end();
}
