use crate::wasm;
pub struct CoreState {
    pub todo_list: Vec<String>,
    pub done_list: Vec<String>,
}

#[derive(Clone)]
pub enum Tab {
    Todo,
    Done,
}

pub struct CuiState {
    pub curr_tab: Tab,
    pub win: wasm::Window,
    pub todo_curs: Option<usize>,
    pub done_curs: Option<usize>,
}

pub enum CuiResponse {
    Quit,
    Shift(Tab, usize),
    AppendTodo(String),
    Edit(String, usize),
    UserInput(String),
}
