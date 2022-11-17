use pancurses as pc;

pub fn start() {
    let window = pc::initscr();
    pc::curs_set(0);
    ui_loop(window);
    pc::endwin();
}

fn ui_loop(win: pc::Window) {
    let mut key_input = None;
    loop {
        if let Some(key) = key_input {
            match key {
                pc::Input::Character('q') => break,
                _ => (),
            }
        }

        render(&win);

        key_input = win.getch();
    }
}

fn render(win: &pc::Window) {
    win.clear();
    win.printw("TODO List:");
    win.refresh();
}
