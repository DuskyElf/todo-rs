use crate::{self as lib, CoreState, CuiState};

pub fn start() {
    let mut core_state = CoreState {
        task_list: vec![],
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
        let response = cui_state.update(key_input, core_state);
        match response {
            lib::CuiResponse::Quit => break,
            lib::CuiResponse::Undo => undo(core_state),
            lib::CuiResponse::UserInput(key) => {
                key_response = true;
                key_input = key;
            }
            lib::CuiResponse::Shift(tab, index) => shift(core_state, tab, index),
            lib::CuiResponse::Edit(new_string, index) => edit(core_state, new_string, index),
            lib::CuiResponse::AppendTodo(new_string) => append(core_state, new_string),
            lib::CuiResponse::Delete(tab, index) => delete(core_state, tab, index),
        }

        // `key_response` takes care to pass the key_input to the next loop iteration
        if !key_response {
            key_input = None;
        }
    }

    cui_state.end();
}

fn undo(core_state: &mut CoreState) {
    let Some(last_task) = core_state.task_list.pop() else {
        return;
    };
    match last_task {
        lib::Task::Edit(last_str, index) => core_state.todo_list[index] = last_str,
        lib::Task::Append(index) => _ = core_state.todo_list.remove(index),
        lib::Task::Shift(last_tab, last_index, index) => {
            match last_tab {
                lib::Tab::Todo => {
                    let element = core_state.done_list.remove(index);
                    core_state.todo_list.insert(last_index, element);
                },
                lib::Tab::Done => {
                    let element = core_state.todo_list.remove(index);
                    core_state.done_list.insert(last_index, element);
                },
            }
        }
        lib::Task::Delete(tab, last_str, index) => {
            match tab {
                lib::Tab::Todo => core_state.todo_list.insert(index, last_str),
                lib::Tab::Done => core_state.done_list.insert(index, last_str),
            }
        }
    }
}

fn shift(core_state: &mut CoreState, tab: lib::Tab, index: usize) {
    match tab {
        lib::Tab::Todo => {
            core_state.task_list.push(lib::Task::Shift(
                tab,
                index,
                core_state.done_list.len()
            ));
            let item = core_state.todo_list.remove(index);
            core_state.done_list.push(item);
        }
        lib::Tab::Done => {
            core_state.task_list.push(lib::Task::Shift(
                tab,
                index,
                core_state.todo_list.len()
            ));
            let item = core_state.done_list.remove(index);
            core_state.todo_list.push(item);
        }
    }
}

fn edit(core_state: &mut CoreState, new_string: String, index: usize) {
    core_state.task_list.push(lib::Task::Edit(
        core_state.todo_list[index].clone(),
        index
    ));
    core_state.todo_list[index] = new_string;
}

fn append(core_state: &mut CoreState, new_string: String) {
    core_state.task_list.push(lib::Task::Append(
        core_state.todo_list.len()
    ));
    core_state.todo_list.push(new_string);
}

fn delete(core_state: &mut CoreState, tab: lib::Tab, index: usize) {
    match tab {
        lib::Tab::Todo => {
            core_state.task_list.push(lib::Task::Delete(
                tab,
                core_state.todo_list[index].clone(),
                index
            ));
            core_state.todo_list.remove(index);
        }
        lib::Tab::Done => {
            core_state.task_list.push(lib::Task::Delete(
                tab,
                core_state.done_list[index].clone(),
                index
            ));
            core_state.done_list.remove(index);
        }
    }
}
