use crate::*;

use Tab::*;
use Task::*;

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
