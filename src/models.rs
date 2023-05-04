use pancurses as pc;

pub enum Task {
    Append(usize),
    Edit(String, usize),
    Shift(Tab, usize, usize),
    Delete(Tab, String, usize),
}

pub struct CoreState {
    pub task_list: Vec<Task>,
    pub todo_list: Vec<String>,
    pub done_list: Vec<String>,
}

#[derive(Clone, Copy)]
pub enum Tab {
    Todo,
    Done,
}

pub struct CuiState {
    pub curr_tab: Tab,
    pub win: pc::Window,
    pub todo_curs: Option<usize>,
    pub done_curs: Option<usize>,
}

pub enum CuiResponse {
    Quit,
    Undo,
    Shift(Tab, usize),
    AppendTodo(String),
    Delete(Tab, usize),
    Edit(String, usize),
    UserInput(Option<pc::Input>),
}
