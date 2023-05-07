use std::fs::{self, DirBuilder};
use std::process::exit;

use serde_json;
use directories as d;

use Tab::*;
use Task::*;

mod models;
pub mod cui;
pub use models::*;

pub fn main() {
    let mut core_state = init_state();
    main_loop(&mut core_state);
    save_state(core_state);
}

fn init_state() -> CoreState {
    let Some(proj_dirs) = d::ProjectDirs::from("me", "DuskyElf", "todo-rs") else {
        eprintln!("Error: Alien Operating System?");
        exit(1);
    };

    let data_file_path = proj_dirs.data_dir().join("data_file.json");
    
    let data = fs::read_to_string(data_file_path);
    let Ok(data) = data else {
        return CoreState {
            task_list: Vec::new(),
            todo_list: Vec::new(),
            done_list: Vec::new(),
        }
    };
    
    let Ok(saved_data) = serde_json::from_str::<SavedData>(&data) else {
        eprintln!("Error: Currepted data_file.");
        exit(1);
    };
    
    CoreState {
        task_list: Vec::new(),
        todo_list: saved_data.todo_list,
        done_list: saved_data.done_list,
    }
}

fn save_state(core_state: CoreState) {
    let Some(proj_dirs) = d::ProjectDirs::from("me", "DuskyElf", "todo-rs") else {
        eprintln!("Error: Alien Operating System?");
        exit(1);
    };

    DirBuilder::new()
        .recursive(true)
        .create(proj_dirs.data_dir()).unwrap();

    let data_file_path = proj_dirs.data_dir().join("data_file.json");

    let data_to_be_saved = SavedData {
        todo_list: core_state.todo_list,
        done_list: core_state.done_list,
    };
    let data_to_be_saved = serde_json::to_string_pretty(&data_to_be_saved).unwrap();
    
    fs::write(data_file_path, data_to_be_saved).unwrap_or_else(|err|
        eprintln!("Error: Can't save the data due to {err}")
    );
}

fn main_loop(core_state: &mut CoreState) {
    let mut key_input = None;
    let mut cui_state = CuiState::init();

    loop {
        let mut key_response = false;
        let response = cui_state.update(key_input, core_state);
        use CuiResponse::*;
        match response {
            Quit => break,
            Undo => core_state.undo(),
            UserInput(key) => {
                key_response = true;
                key_input = key;
            }
            Shift(tab, index) => core_state.shift(tab, index),
            Edit(new_string, index) => core_state.edit(new_string, index),
            AppendTodo(new_string) => core_state.append(new_string),
            Delete(tab, index) => core_state.delete(tab, index),
        }

        // `key_response` takes care to pass the key_input to the next loop iteration
        if !key_response {
            key_input = None;
        }
    }

    cui_state.end();
}

impl CoreState {
    fn undo(&mut self) {
        let Some(last_task) = self.task_list.pop() else {
            return;
        };
        match last_task {
            Edit(last_str, index) => self.todo_list[index] = last_str,
            Append(index) => _ = self.todo_list.remove(index),
            Shift(last_tab, last_index, index) => {
                match last_tab {
                    Todo => {
                        let element = self.done_list.remove(index);
                        self.todo_list.insert(last_index, element);
                    },
                    Done => {
                        let element = self.todo_list.remove(index);
                        self.done_list.insert(last_index, element);
                    },
                }
            }
            Delete(tab, last_str, index) => {
                match tab {
                    Todo => self.todo_list.insert(index, last_str),
                    Done => self.done_list.insert(index, last_str),
                }
            }
        }
    }

    fn shift(&mut self, tab: Tab, index: usize) {
        match tab {
            Todo => {
                self.task_list.push(Task::Shift(
                    tab,
                    index,
                    self.done_list.len()
                ));
                let item = self.todo_list.remove(index);
                self.done_list.push(item);
            }
            Done => {
                self.task_list.push(Task::Shift(
                    tab,
                    index,
                    self.todo_list.len()
                ));
                let item = self.done_list.remove(index);
                self.todo_list.push(item);
            }
        }
    }

    fn edit(&mut self, new_string: String, index: usize) {
        self.task_list.push(Task::Edit(
            self.todo_list[index].clone(),
            index
        ));
        self.todo_list[index] = new_string;
    }

    fn append(&mut self, new_string: String) {
        self.task_list.push(Task::Append(
            self.todo_list.len()
        ));
        self.todo_list.push(new_string);
    }

    fn delete(&mut self, tab: Tab, index: usize) {
        match tab {
            Todo => {
                self.task_list.push(Task::Delete(
                    tab,
                    self.todo_list[index].clone(),
                    index
                ));
                self.todo_list.remove(index);
            }
            Done => {
                self.task_list.push(Task::Delete(
                    tab,
                    self.done_list[index].clone(),
                    index
                ));
                self.done_list.remove(index);
            }
        }
    }
}
