use pancurses as pc;

pub struct CoreState {
    pub todo_list: Vec<String>,
    pub done_list: Vec<String>,
}

pub enum Tab {
    Todo,
    Done,
}

pub struct CuiState {
    pub curr_tab: Tab,
    pub win: pc::Window,
    pub todo_curs: usize,
    pub done_curs: usize,
}

pub enum CuiResponse {
    Quit,
    Shift(Tab, usize),
    UserInput(Option<pc::Input>),
}
