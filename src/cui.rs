use pancurses as pc;
use crate::{Tab, CuiResponse, CuiState, CoreState};

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
        pc::curs_set(0);
        CuiState {
            win,
            curr_tab: Tab::Todo,
            todo_curs: 0,
            done_curs: 0,
        }
    }

    pub fn end(&self) {
        pc::endwin();
    }

    pub fn update(&mut self, key_input: Option<pc::Input>, core_state: &CoreState) -> CuiResponse {
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

    fn handle_input(&mut self, key: pc::Input, core_state: &CoreState) -> Option<CuiResponse> {
        match key {
            pc::Input::Character('q')  => return Some(CuiResponse::Quit),
            pc::Input::Character('\t') => self.curr_tab.toggle(),
            pc::Input::Character('k')  => self.cursor_up(),
            pc::Input::Character('j')  => self.cursor_down(core_state),
            pc::Input::Character('\n') => self.select(),
            _ => (),
        }

        None
    }

    fn cursor_up(&mut self) {
        match self.curr_tab {
            Tab::Todo
                if self.todo_curs > 0 => self.todo_curs -= 1,
            Tab::Done
                if self.done_curs > 0 => self.done_curs -= 1,
            _ => (),
        }
    }

    fn cursor_down(&mut self, core_state: &CoreState) {
        match self.curr_tab {
            Tab::Todo
                if self.todo_curs < core_state.todo_list.len() - 1
                    => self.todo_curs += 1,
            Tab::Done
                if self.todo_curs < core_state.done_list.len() - 1
                    => self.todo_curs += 1,
            _ => (),
        }
    }

    fn select(&mut self) {
    }

    fn render(&self, core_state: &CoreState) {
        self.win.clear();
        self.win.printw("Simple Todo App:\n");
        self.win.printw("------------------\n");

        match self.curr_tab {
            Tab::Todo => {
                self.win.printw("[ Todo ]  Done\n\n");
                self.render_list(&core_state.todo_list, self.todo_curs);
            }
            Tab::Done => {
                self.win.printw("  Todo  [ Done ]\n\n");
                self.render_list(&core_state.done_list, self.done_curs);
            }
        }

        self.win.refresh();
    }

    fn render_list(&self, list: &Vec<String>, cursor: usize) {
        for (i, element) in list.iter().enumerate() {
            if i == cursor {
                self.win.printw(format!("-> | {element}\n"));
            }
            else {
                self.win.printw(format!("  | {element}\n"));
            }
        }
    }
}

