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
    let mut cui_state = CuiState::init();

    loop {
        let responce = cui_state.update(key_input, core_state);
        match responce {
            lib::CuiResponse::Quit => break,
            lib::CuiResponse::UserInput(key) => {
                key_input = key;
            }
            lib::CuiResponse::Shift(tab, index) => shift(core_state, tab, index),
        }
    }

    cui_state.end();
}

fn shift(core_state: &mut CoreState, tab: lib::Tab, index: usize) {
    todo!()
}
