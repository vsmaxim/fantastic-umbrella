use crossterm::event::{self, Event, KeyCode};

use crate::{components::{
    list::List,
    element::Element,
    block::Block, tty::PtyView,
}, console::Console};

pub struct Application {}

impl Application {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) -> std::io::Result<()> {
        let mut console = Console::new();
        let mut pty_view = PtyView::new(20, 80);

        let mut left_pane = Block::new(0, 0, 40, 40);
        let mut right_pane = Block::new(41, 0, 80, 40);

        let mut options_list = List::new(
            vec![
                "Some option".to_string(),
                "Other option".to_string(),
                "Option 3".to_string(),
            ],
        );

        console.enter_full_screen();

        loop {
            options_list.output(
                &mut console,
                &mut left_pane,
            );

            pty_view.output(
                &mut console,
                &mut right_pane,
            );

            console.flush();

            if event::poll(std::time::Duration::from_millis(100))? {
                let event = event::read()?;
                options_list.on_event(&event)?;

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
