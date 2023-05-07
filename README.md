# ToDo RS
A simple lightweight CUI todo app in Rust.

## Quick Start
Note:- Rust tool-chain should be installed on your machine.
### Installation via cargo install
```shell
$ cargo install de-todo
```
### Cloning with git and compiling
```shell
$ git clone https://github.com/DuskyElf/todo-rs.git
$ cd todo-rs
$ cargo r --release
```

## Screenshots
![App Screenshot](https://user-images.githubusercontent.com/91879372/204074472-1a53fdb7-f8d5-4ea5-9c6b-ce5740c22a51.png)

## Features
- Simple CUI
- Add new TODOs
- Mark TODOs complete, to shift to done list
- Edit TODOs
- Delete TODOs
- Undo last changes
- Auto Save and load Todo and Done list on quit and restart
- Fullscreen mode

## Quick Guide
### Keybinds
- `q` - Quit  `X`
- `Tab` - Toggle between tabs  `⇆`
- `k` - Move up in list `↑` 
- `j` - Move down in list `↓`
- `i` - Edit the selected Todo
- `a` - Add a new Todo
- `d` - Delete the selected element
- `u` - Undo last task (tasks includes `edit`, `append`, `delete` and `shift`)
- `Enter` `return` - Shift item to another tab `↵`

## Cargo Dependencies
- pancurses
- directories
- serde
- serde_json
