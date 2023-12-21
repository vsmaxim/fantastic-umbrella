use crossterm::event::{self, Event, KeyCode};

use crate::{components::{
    list::List,
    element::Element,
    block::Block, tty::PtyView, input::Input,
}, console::Console};

pub struct Application {
    select_mode: bool, 
}

impl Application {
    pub fn new() -> Self {
        Self {
            select_mode: false,
        }
    }

    pub fn run(&self) -> std::io::Result<()> {
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


        left_pane.render(&mut console);

        loop {
            pty_view.output(
                &mut console,
                &mut right_pane,
            );
            right_pane.render(&mut console);

            address_input.output(
                &mut console,
                &mut input_block,
            );

            input_block.render(&mut console);

            console.flush();

            if event::poll(std::time::Duration::from_millis(100))? {
                let event = event::read()?;

                address_input.on_event(&event)?;
                // options_list.on_event(&event)?;

                if let Event::Key(key_event) = event {
                    if key_event.code == KeyCode::Esc {
                        break;
                    }
                }
            }
        }

        console.exit_full_screen();

        Ok(())
    }
}
