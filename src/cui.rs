use pancurses as pc;
use crate::{Tab, CuiResponse, CuiState, CoreState};

const CUI_OFFSET_Y: i32 = 4;
const CUI_OFFSET_X: i32 = 5;

const TEXT_COLOR:       u32 = 1;
const TITLE_COLOR:      u32 = 2;
const SELECTED_COLOR:   u32 = 3;
const INDICATOR_COLOR:  u32 = 4;
const INSERTMODE_COLOR: u32 = 5;

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
        let win = pc::initscr();
        if pc::has_colors() {
            pc::start_color();
        }

        // COLORS
        pc::init_pair(TEXT_COLOR as i16, pc::COLOR_WHITE, pc::COLOR_BLACK);
        pc::init_pair(TITLE_COLOR as i16, pc::COLOR_BLUE, pc::COLOR_BLACK);
        pc::init_pair(SELECTED_COLOR as i16, pc::COLOR_BLUE, pc::COLOR_BLACK);
        pc::init_pair(INDICATOR_COLOR as i16, pc::COLOR_RED, pc::COLOR_BLACK);
        pc::init_pair(INSERTMODE_COLOR as i16, pc::COLOR_GREEN, pc::COLOR_BLACK);

        pc::noecho();
        pc::curs_set(0);
        win.keypad(true);
        CuiState {
            win,
            curr_tab: Tab::Todo,
            todo_curs: None,
            done_curs: None,
        }
    }

    pub fn end(&self) {
        pc::endwin();
    }

    pub fn update(&mut self, key_input: Option<pc::Input>, core_state: &CoreState) -> CuiResponse {
        self.init_cursor(core_state);

        if let Some(key) = key_input {
            if let Some(response) = self.handle_input(key, core_state) {
                return response;
            }
        }

        self.render(core_state);

        CuiResponse::UserInput(
            self.win.getch()
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

    fn handle_input(&mut self, key: pc::Input, core_state: &CoreState) -> Option<CuiResponse> {
        match key {
            pc::Input::Character('q')  => return Some(CuiResponse::Quit),
            pc::Input::Character('\t') => self.curr_tab.toggle(),
            pc::Input::Character('k')  => self.cursor_up(),
            pc::Input::Character('j')  => self.cursor_down(core_state),
            pc::Input::Character('\n') => {
                if let Some(index) = self.handle_selection() {
                    return Some(CuiResponse::Shift(self.curr_tab.clone(), index))
                }
            }
            pc::Input::Character('i')  => {
                if let Tab::Todo = self.curr_tab {
                    if let Some(new_string) = self.edit(core_state) {
                        return Some(CuiResponse::Edit(new_string, self.todo_curs.unwrap()));
                    }
                }
            }
            pc::Input::Character('a')  => {
                if let Tab::Todo = self.curr_tab {
                    if let Some(new_string) = self.append(core_state) {
                        return Some(CuiResponse::AppendTodo(new_string));
                    }
                }
            }
            _ => (),
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
        pc::curs_set(1);
        self.win.attron(pc::COLOR_PAIR(INSERTMODE_COLOR));
        loop {
            match self.win.getch().unwrap() {
                pc::Input::KeyExit => return None,
                pc::Input::Character('\n') => {
                    break;
                }

                pc::Input::KeyBackspace => {
                    if buffer.len() != 0 {
                        buffer.pop();
                        self.win.mv(self.win.get_cur_y(), self.win.get_cur_x() - 1);
                        self.win.delch();
                    }
                    continue;
                }

                pc::Input::Character(read) => {
                    self.win.addch(read);
                    buffer.push(read);
                }

                _ => (),
            }
        }
        self.win.attron(pc::COLOR_PAIR(TEXT_COLOR));
        pc::curs_set(0);
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

    fn render(&self, core_state: &CoreState) {
        assert_eq!(CUI_OFFSET_Y, 4);
        self.win.clear();
        self.win.attron(pc::COLOR_PAIR(TITLE_COLOR) | pc::A_BOLD);
        self.win.printw("Simple Todo App:\n");
        self.win.printw("------------------\n");
        self.win.attroff(pc::A_BOLD);
        self.win.attron(pc::COLOR_PAIR(TEXT_COLOR));

        match self.curr_tab {
            Tab::Todo => {
                self.win.attron(pc::COLOR_PAIR(INDICATOR_COLOR));
                self.win.addch('[');

                self.win.attron(pc::COLOR_PAIR(SELECTED_COLOR));
                self.win.printw(" Todo ");

                self.win.attron(pc::COLOR_PAIR(INDICATOR_COLOR));
                self.win.addch(']');
                self.win.attron(pc::COLOR_PAIR(TEXT_COLOR));

                self.win.printw("  Done\n\n");
                self.render_list(&core_state.todo_list, self.todo_curs);
            }
            Tab::Done => {
                self.win.printw("  Todo  ");

                self.win.attron(pc::COLOR_PAIR(INDICATOR_COLOR));
                self.win.addch('[');

                self.win.attron(pc::COLOR_PAIR(SELECTED_COLOR));
                self.win.printw(" Done ");

                self.win.attron(pc::COLOR_PAIR(INDICATOR_COLOR));
                self.win.addch(']');
                self.win.printw("\n\n");
                self.win.attron(pc::COLOR_PAIR(TEXT_COLOR));

                self.render_list(&core_state.done_list, self.done_curs);
            }
        }

        self.win.refresh();
    }

    fn render_list(&self, list: &Vec<String>, cursor: Option<usize>) {
        let Some(cursor) = cursor else {
            assert_eq!(list.len(), 0);
            return;
        };
        assert_eq!(CUI_OFFSET_X, 5);
        for (i, element) in list.iter().enumerate() {
            if i == cursor {
                self.win.attron(pc::COLOR_PAIR(INDICATOR_COLOR));
                self.win.printw("-> | ");
                self.win.attron(pc::COLOR_PAIR(SELECTED_COLOR));
                self.win.printw(format!("{element}\n"));
                self.win.attron(pc::COLOR_PAIR(TEXT_COLOR));
            }
            else {
                self.win.printw(format!("  | {element}\n"));
            }
        }
    }
}

