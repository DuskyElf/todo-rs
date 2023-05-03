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
        let mut key_response = false;
        let response = cui_state.update(key_input.clone(), core_state);
        match response {
            lib::CuiResponse::Quit => break,
            lib::CuiResponse::UserInput(key) => {
                key_response = true;
                key_input = Some(key);
            }
            lib::CuiResponse::Shift(tab, index) => shift(core_state, tab, index),
            lib::CuiResponse::Edit(new_string, index) => edit(core_state, new_string, index),
            lib::CuiResponse::AppendTodo(new_string) => append(core_state, new_string),
        }

        // `key_response` takes care to pass the key_input to the next loop iteration
        if !key_response {
            key_input = None;
        }
    }
}

fn shift(core_state: &mut CoreState, tab: lib::Tab, index: usize) {
    match tab {
        lib::Tab::Todo => {
            let item = core_state.todo_list.remove(index);
            core_state.done_list.push(item);
        }
        lib::Tab::Done => {
            let item = core_state.done_list.remove(index);
            core_state.todo_list.push(item);
        }
    }
}

fn edit(core_state: &mut CoreState, new_string: String, index: usize) {
    core_state.todo_list[index] = new_string;
}

fn append(core_state: &mut CoreState, new_string: String) {
    core_state.todo_list.push(new_string);
}
