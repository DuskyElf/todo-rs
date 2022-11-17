use pancurses as pc;

struct CuiState {
    curr_tab: Tab,
}

enum Tab {
    Todo,
    Done,
}

impl Tab {
    fn toggle(&mut self) {
        *self = match *self {
            Self::Todo => Self::Done,
            Self::Done => Self::Todo,
        }
    }
}

pub fn start() {
    let window = pc::initscr();
    pc::curs_set(0);
    ui_loop(window);
    pc::endwin();
}

fn ui_loop(win: pc::Window) {
    let mut cui_state = CuiState {
        curr_tab: Tab::Todo,
    };

    let mut key_input = None;
    loop {
        if let Some(key) = key_input {
            match key {
                pc::Input::Character('q') => break,
                pc::Input::Character('\t') => cui_state.curr_tab.toggle(),
                _ => (),
            }
        }

        render(&win, &cui_state);

        key_input = win.getch();
    }
}

fn render(win: &pc::Window, cui_state: &CuiState) {
    win.clear();
    win.printw("Simple Todo App:\n\n");

    match cui_state.curr_tab {
        Tab::Todo => {
            win.printw("[ Todo ]  Done");
        }
        Tab::Done => {
            win.printw("  Todo  [ Done ]");
        }
    }

    win.refresh();
}
