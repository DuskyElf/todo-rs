use todo_rs;
use pancurses::{initscr, endwin};

fn main() {
    let window = initscr();
    window.printw("TODO List:");
    window.refresh();
    window.getch();
    endwin();
}
