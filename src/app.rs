use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{components::{
    list::List,
    element::Element,
    block::{Block, BlockState}, tty::PtyView, input::Input,
}, console::Console};

pub struct Application {
    select_mode: bool, 
}

impl Application {
    pub fn new() -> Self {
        Self {
            select_mode: true,
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut console = Console::new();

        let mut left_pane = Block::new(0, 0, 40, 80, true);

        let mut options_list = List::new(
            vec![
                "Some option".to_string(),
                "Other option".to_string(),
                "Option 3".to_string(),
            ],
        );

        let mut address_input = Input::new();
        let mut input_block = Block::new(42, 0, 82, 3, true);

        let mut right_pane = Block::new(42, 3, 82, 22, true);
        let mut pty_view = PtyView::new(20, 80);

        console.enter_full_screen();

        options_list.output(
            &mut console,
            &mut left_pane,
        );

        left_pane.set_state(BlockState::Selected);
        left_pane.render(&mut console);

        loop {
            options_list.output(&mut console, &mut left_pane);

            pty_view.output(&mut console, &mut right_pane);
            right_pane.render(&mut console);

            address_input.output(&mut console, &mut input_block);
            input_block.render(&mut console);

            console.flush();

            if event::poll(std::time::Duration::from_millis(100))? {
                let event = event::read()?;

                if let Event::Key(KeyEvent { 
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) = event {
                    if self.select_mode {
                        match code {
                            KeyCode::Up => {
                                if right_pane.is_selected() {
                                    right_pane.set_state(BlockState::Inactive);
                                    input_block.set_state(BlockState::Selected);

                                    right_pane.render(&mut console);
                                    input_block.render(&mut console);
                                }
                            },
                            KeyCode::Down => {
                                if input_block.is_selected() {
                                    input_block.set_state(BlockState::Inactive);
                                    right_pane.set_state(BlockState::Selected);

                                    input_block.render(&mut console);
                                    right_pane.render(&mut console);
                                }
                            },
                            KeyCode::Left => {
                                if right_pane.is_selected() {
                                    right_pane.set_state(BlockState::Inactive);
                                    left_pane.set_state(BlockState::Selected);

                                    right_pane.render(&mut console);
                                    left_pane.render(&mut console);
                                } else if input_block.is_selected() {
                                    input_block.set_state(BlockState::Inactive);
                                    left_pane.set_state(BlockState::Selected);

                                    input_block.render(&mut console);
                                    left_pane.render(&mut console);
                                }
                            },
                            KeyCode::Right => {
                                if left_pane.is_selected() {
                                    left_pane.set_state(BlockState::Inactive);
                                    input_block.set_state(BlockState::Selected);

                                    left_pane.render(&mut console);
                                    input_block.render(&mut console);
                                }
                            },
                            KeyCode::Enter => {
                                if left_pane.is_selected() {
                                    left_pane.set_state(BlockState::Active);
                                    left_pane.render(&mut console);
                                } 

                                if right_pane.is_selected() {
                                    right_pane.set_state(BlockState::Active);
                                    right_pane.render(&mut console);
                                }

                                if input_block.is_selected() {
                                    input_block.set_state(BlockState::Active);
                                    input_block.render(&mut console);
                                }

                                self.select_mode = false;
                            },
                            KeyCode::Esc => break,  
                            _ => {}
                        }
                    } else {
                        match code {
                            KeyCode::Esc => {
                                if left_pane.is_active() {
                                    left_pane.set_state(BlockState::Selected);
                                    left_pane.render(&mut console);
                                } 

                                if right_pane.is_active() {
                                    right_pane.set_state(BlockState::Selected);
                                    right_pane.render(&mut console);
                                }

                                if input_block.is_active() {
                                    input_block.set_state(BlockState::Selected);
                                    input_block.render(&mut console);
                                }

                                self.select_mode = true;
                            },
                            _ => {
                                if left_pane.is_active() {
                                    options_list.on_event(&event)?;
                                } else if right_pane.is_active() {  
                                    pty_view.on_event(&event)?;
                                } else if input_block.is_active() {
                                    address_input.on_event(&event)?;
                                }
                            }
                        };
                    }
                }
            }
        }

        console.exit_full_screen();

        Ok(())
    }
}
