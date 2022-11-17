use crate::{self as lib, cui, CoreState};

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

    main_loop();
}

fn main_loop() {
    let mut key_input = None;
    let mut cui_state = cui::init();

    loop {
        let responce = cui::update(&mut cui_state, key_input);
        match responce {
            lib::CuiResponse::Quit => break,
            lib::CuiResponse::UserInput(key) => {
                key_input = key;
            }
        }
    }

    cui::end()
}
