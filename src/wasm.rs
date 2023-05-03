use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn js_get_ch() -> String;
    fn js_refresh(value: String);
}

pub struct Window {
    buffer: Vec<String>,
    cur_x: i32,
    cur_y: i32,
}

impl Window {
    pub fn new() -> Self {
        Window {
            buffer: vec![String::new()],
            cur_x: 0,
            cur_y: 0,
        }
    }

    pub fn get_ch(&mut self) -> String {
        js_get_ch()
    }
    
    pub fn add_str(&mut self, value: &str) {
        if self.cur_y as usize >= self.buffer.len() {
            for _ in 0..(self.buffer.len() - self.cur_y as usize + 1) {
                self.buffer.push(String::new());
            }
        }
        let cur_str = &mut self.buffer[self.cur_y as usize];
        if self.cur_x as usize >= cur_str.len() {
            for _ in 0..(cur_str.len() - self.cur_x as usize + 1) {
                cur_str.push(' ')
            }
        }
        let left = &cur_str[0..(self.cur_x as usize)].to_string();
        let right = &cur_str[(self.cur_x as usize)..cur_str.len()].to_string();
        cur_str.clear();
        cur_str.push_str(left);
        cur_str.push_str(value);
        cur_str.push_str(right);
    }

    pub fn mv(&mut self, x: i32, y: i32) {
        self.cur_x = x;
        self.cur_y = y;
    }

    pub fn printw(&mut self, value: &str) {
        self.add_str(value);
    }

    pub fn set_color(&mut self, value: &str) {
        self.add_str(&format!("</span><span style=\"{value}\">"));
    }
    
    pub fn get_cur_y(&self) -> i32 {
        self.cur_y
    }

    pub fn get_cur_x(&self) -> i32 {
        self.cur_x
    }

    pub fn delch(&mut self) {
        self.buffer[self.cur_y as usize].remove(self.cur_x as usize);
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn refresh(&mut self) {
        js_refresh(self.buffer.join("\n"));
    }
}
