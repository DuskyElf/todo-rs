use pancurses as pc;
use crate::{Tab, CuiResponse, CuiState};

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
        }
    }

    pub fn end() {
        pc::endwin();
    }

    pub fn update(&mut self, key_input: Option<pc::Input>) -> CuiResponse {
        if let Some(key) = key_input {
            // `handle_input` returns false to exit the ui_loop
            if !self.handle_input(key) {
                return CuiResponse::Quit;
            }
        }

        self.render();

        CuiResponse::UserInput(
            self.win.getch()
        )
    }

    // Returns false to exit the ui_loop
    fn handle_input(&mut self, key: pc::Input) -> bool {
        match key {
            pc::Input::Character('q') => return false,
            pc::Input::Character('\t') => self.curr_tab.toggle(),
            _ => (),
        }

        true
    }

    fn render(&self) {
        self.win.clear();
        self.win.printw("Simple Todo App:\n\n");

        match self.curr_tab {
            Tab::Todo => {
                self.win.printw("[ Todo ]  Done");
            }
            Tab::Done => {
                self.win.printw("  Todo  [ Done ]");
            }
        }

        self.win.refresh();
    }
}

