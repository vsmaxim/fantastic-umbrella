use crossterm::{terminal, event::{KeyEvent, Event, KeyEventKind, KeyCode}};
use crate::{components::block::{Block, BlockState}, console::Console};


pub struct Layout {
    pub select_mode: bool,
    pub list_cont: Block,
    pub input_cont: Block,
    pub req_cont: Block,
    pub hint_cont: Block,
}


impl Layout {
    pub fn new() -> Self {
        let (width, height) = terminal::size().expect("Couldn't get terminal size");

        let left_col_width: u16 = 40;
        let right_col_width: u16 = width - left_col_width - 2;

        let input_height: u16 = 3;
        let hint_height: u16 = 3;
        let list_height: u16 = height - hint_height;
        let req_height: u16 = height - hint_height - input_height;

        let mut list_cont = Block::new(0, 0, left_col_width, list_height, true);
        list_cont.set_state(BlockState::Selected);

        let input_cont = Block::new(left_col_width + 1, 0, right_col_width, input_height, true);
        let req_cont = Block::new(left_col_width + 1, input_height, right_col_width, req_height, true);
        let hint_cont = Block::new(0, list_height, width - 1, hint_height, true);


        Self {
            select_mode: true,
            list_cont,
            input_cont,
            req_cont,
            hint_cont,
        }
    }

    pub fn enter_select_mode(&mut self) {
        self.select_mode = true;

        if self.input_cont.is_active() {
            self.input_cont.set_state(BlockState::Selected);
        }

        if self.req_cont.is_active() {
            self.req_cont.set_state(BlockState::Selected);
        }

        if self.list_cont.is_active() {
            self.list_cont.set_state(BlockState::Selected);
        }
    }
    
    pub fn render(&mut self, console: &mut Console) {
        self.list_cont.render(console);
        self.input_cont.render(console);
        self.req_cont.render(console);
        self.hint_cont.render(console);
    }

    pub fn navigate(&mut self, e: &Event, console: &mut Console) {
        if let Event::Key(
            KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }
        ) = e {
            if self.select_mode {
                match code {
                    KeyCode::Up => {
                        if self.req_cont.is_selected() {
                            self.req_cont.set_state(BlockState::Inactive);
                            self.input_cont.set_state(BlockState::Selected);
                            self.req_cont.render(console);
                            self.input_cont.render(console);
                        }
                    },
                    KeyCode::Down => {
                        if self.input_cont.is_selected() {
                            self.input_cont.set_state(BlockState::Inactive);
                            self.req_cont.set_state(BlockState::Selected);
                            self.input_cont.render(console);
                            self.req_cont.render(console);
                        }
                    },
                    KeyCode::Left => {
                        if self.req_cont.is_selected() {
                            self.req_cont.set_state(BlockState::Inactive);
                            self.list_cont.set_state(BlockState::Selected);

                            self.req_cont.render(console);
                            self.list_cont.render(console);
                        } else if self.input_cont.is_selected() {
                            self.input_cont.set_state(BlockState::Inactive);
                            self.list_cont.set_state(BlockState::Selected);

                            self.input_cont.render(console);
                            self.list_cont.render(console);
                        }
                    },
                    KeyCode::Right => {
                        if self.list_cont.is_selected() {
                            self.list_cont.set_state(BlockState::Inactive);
                            self.input_cont.set_state(BlockState::Selected);

                            self.list_cont.render(console);
                            self.input_cont.render(console);
                        }
                    },
                    KeyCode::Enter => {
                        if self.list_cont.is_selected() {
                            self.list_cont.set_state(BlockState::Active);
                            self.list_cont.render(console);
                        } 

                        if self.req_cont.is_selected() {
                            console.show_cursor();
                            self.req_cont.set_state(BlockState::Active);
                            self.req_cont.render(console);
                        }

                        if self.input_cont.is_selected() {
                            self.input_cont.set_state(BlockState::Active);
                            self.input_cont.render(console);
                        }

                        self.select_mode = false;
                    },
                    _ => {}
                }
            }
        }
    }
}

