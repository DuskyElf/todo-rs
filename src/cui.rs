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

pub fn init() -> CuiState {
    let win = pc::initscr();
    pc::curs_set(0);
    CuiState {
        win,
        curr_tab: Tab::Todo,
    }
}

pub fn update(cui_state: &mut CuiState, key_input: Option<pc::Input>) -> CuiResponse {
    if let Some(key) = key_input {
        // `handle_input` returns false to exit the ui_loop
        if !handle_input(key, cui_state) {
            return CuiResponse::Quit;
        }
    }

    render(&cui_state);

    CuiResponse::UserInput(
        cui_state.win.getch()
    )
}

pub fn end() {
    pc::endwin();
}

// Returns false to exit the ui_loop
fn handle_input(key: pc::Input, cui_state: &mut CuiState) -> bool {
    match key {
        pc::Input::Character('q') => return false,
        pc::Input::Character('\t') => cui_state.curr_tab.toggle(),
        _ => (),
    }

    true
}

fn render(cui_state: &CuiState) {
    cui_state.win.clear();
    cui_state.win.printw("Simple Todo App:\n\n");

    match cui_state.curr_tab {
        Tab::Todo => {
            cui_state.win.printw("[ Todo ]  Done");
        }
        Tab::Done => {
            cui_state.win.printw("  Todo  [ Done ]");
        }
    }

    cui_state.win.refresh();
}
