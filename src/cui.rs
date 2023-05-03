use crate::{Tab, CuiResponse, CuiState, CoreState, wasm};

const CUI_OFFSET_Y: i32 = 4;
const CUI_OFFSET_X: i32 = 5;

const TEXT_COLOR:       &str = "white";
const TITLE_COLOR:      &str = "blue";
const SELECTED_COLOR:   &str = "blue";
const INDICATOR_COLOR:  &str = "red";
const INSERTMODE_COLOR: &str = "green";

impl Tab {
    fn toggle(&mut self) {
        *self = match *self {
            Self::Todo => Self::Done,
            Self::Done => Self::Todo,
        }
    }
}

impl CuiState {
    pub fn init() -> CuiState {
        let window = wasm::Window::new();

        CuiState {
            win: window,
            curr_tab: Tab::Todo,
            todo_curs: None,
            done_curs: None,
        }
    }

    pub fn update(&mut self, key_input: Option<String>, core_state: &CoreState) -> CuiResponse {
        self.init_cursor(core_state);

        if let Some(key) = key_input {
            if let Some(response) = self.handle_input(key, core_state) {
                return response;
            }
        }

        self.render(core_state);

        CuiResponse::UserInput(
            self.win.get_ch()
        )
    }

    fn init_cursor(&mut self, core_state: &CoreState) {
        if let None = self.todo_curs {
            if core_state.todo_list.len() != 0 {
                self.todo_curs = Some(0);
            }
        }
        if let None = self.done_curs {
            if core_state.done_list.len() != 0 {
                self.done_curs = Some(0);
            }
        }
    }

    fn handle_input(&mut self, key: String, core_state: &CoreState) -> Option<CuiResponse> {
        if key == "q" { return Some(CuiResponse::Quit); }
        else if key == "\t" { self.curr_tab.toggle() }
        else if key == "k" { self.cursor_up() }
        else if key == "j" { self.cursor_down(core_state) }
        else if key == "\n" {
            if let Some(index) = self.handle_selection() {
                return Some(CuiResponse::Shift(self.curr_tab.clone(), index))
            }
        }
        else if key == "i" {
            if let Tab::Todo = self.curr_tab {
                if let Some(new_string) = self.edit(core_state) {
                    return Some(CuiResponse::Edit(new_string, self.todo_curs.unwrap()));
                }
            }
        }
        else if key == "a" {
            if let Tab::Todo = self.curr_tab {
                if let Some(new_string) = self.append(core_state) {
                    return Some(CuiResponse::AppendTodo(new_string));
                }
            }
        }

        None
    }

    fn cursor_up(&mut self) {
        match self.curr_tab {
            Tab::Todo => {
                let Some(todo_curs) = &mut self.todo_curs else {
                    return;
                };
                if *todo_curs > 0 {
                    *todo_curs -= 1;
                }
            }
            Tab::Done => {
                let Some(done_curs) = &mut self.done_curs else {
                    return;
                };
                if *done_curs > 0 {
                    *done_curs -= 1;
                }
            }
        }
    }

    fn cursor_down(&mut self, core_state: &CoreState) {
        match self.curr_tab {
            Tab::Todo => {
                let Some(todo_curs) = &mut self.todo_curs else {
                    return;
                };
                if *todo_curs < core_state.todo_list.len() - 1 {
                    *todo_curs += 1;
                }
            }
            Tab::Done => {
                let Some(done_curs) = &mut self.done_curs else {
                    return;
                };
                if *done_curs < core_state.done_list.len() - 1 {
                    *done_curs += 1;
                }
            }
        }
    }

    fn edit(&mut self, core_state: &CoreState) -> Option<String> {
        let Some(todo_curs) = &mut self.todo_curs else {
            return None;
        };

        let mut buffer = core_state.todo_list[*todo_curs].clone();
        self.win.mv(
            *todo_curs as i32 + CUI_OFFSET_Y,
            buffer.len() as i32 + CUI_OFFSET_X,
        );
        self.insert_mode(&mut buffer)?;
        Some(buffer)
    }

    fn append(&mut self, core_state: &CoreState) -> Option<String> {
        let mut buffer = String::new();
        self.win.mv(
            core_state.todo_list.len() as i32 + CUI_OFFSET_Y,
            0,
        );
        self.win.printw("> ");
        self.insert_mode(&mut buffer)?;
        self.todo_curs = Some(core_state.todo_list.len());
        Some(buffer)
    }

    fn insert_mode(&mut self, buffer: &mut String) -> Option<()> {
        self.win.set_color(INSERTMODE_COLOR);
        loop {
            let input = self.win.get_ch();
            if input == "Escape" { return None }
            else if input == "\n" { break; }
            else if input == "Backspace" {
                if buffer.len() != 0 {
                    buffer.pop();
                    self.win.mv(self.win.get_cur_y(), self.win.get_cur_x() - 1);
                    self.win.delch();
                }
                continue;
            }
            let input = self.win.get_ch();
            if input == "Backspace" {
                if buffer.len() != 0 {
                    buffer.pop();
                    self.win.mv(self.win.get_cur_y(), self.win.get_cur_x() - 1);
                    self.win.delch();
                }
                continue;
            }
            else if input.len() == 1 {
                self.win.add_str(&input);
                buffer.push_str(&input);
            }
        }
        self.win.set_color(TEXT_COLOR);
        Some(())
    }

    fn handle_selection(&mut self) -> Option<usize> {
        match self.curr_tab {
            Tab::Todo => {
                let Some(todo_curs) = &mut self.todo_curs else {
                    return None;
                };
                let index = *todo_curs;
                if *todo_curs > 0 {
                    *todo_curs -= 1;
                } else {
                    self.todo_curs = None;
                }
                return Some(index);
            }

            Tab::Done => {
                let Some(done_curs) = &mut self.done_curs else {
                    return None;
                };
                let index = *done_curs;
                if *done_curs > 0 {
                    *done_curs -= 1;
                } else {
                    self.done_curs = None;
                }
                return Some(index);
            }
        }
    }

    fn render(&mut self, core_state: &CoreState) {
        assert_eq!(CUI_OFFSET_Y, 4);
        self.win.clear();
        self.win.set_color(TITLE_COLOR);
        self.win.printw("Simple Todo App:\n");
        self.win.printw("------------------\n");
        self.win.set_color(TEXT_COLOR);

        match self.curr_tab {
            Tab::Todo => {
                self.win.set_color(INDICATOR_COLOR);
                self.win.add_str("[");

                self.win.set_color(SELECTED_COLOR);
                self.win.printw(" Todo ");

                self.win.set_color(INDICATOR_COLOR);
                self.win.add_str("]");
                self.win.set_color(TEXT_COLOR);

                self.win.printw("  Done\n\n");
                self.render_list(&core_state.todo_list, self.todo_curs);
            }
            Tab::Done => {
                self.win.printw("  Todo  ");

                self.win.set_color(INDICATOR_COLOR);
                self.win.add_str("[");

                self.win.set_color(SELECTED_COLOR);
                self.win.printw(" Done ");

                self.win.set_color(INDICATOR_COLOR);
                self.win.add_str("]");
                self.win.printw("\n\n");
                self.win.set_color(TEXT_COLOR);

                self.render_list(&core_state.done_list, self.done_curs);
            }
        }

        self.win.refresh();
    }

    fn render_list(&mut self, list: &Vec<String>, cursor: Option<usize>) {
        let Some(cursor) = cursor else {
            assert_eq!(list.len(), 0);
            return;
        };
        assert_eq!(CUI_OFFSET_X, 5);
        for (i, element) in list.iter().enumerate() {
            if i == cursor {
                self.win.set_color(INDICATOR_COLOR);
                self.win.printw("-> | ");
                self.win.set_color(SELECTED_COLOR);
                self.win.printw(&format!("{element}\n"));
                self.win.set_color(TEXT_COLOR);
            }
            else {
                self.win.printw(&format!("  | {element}\n"));
            }
        }
    }
}

