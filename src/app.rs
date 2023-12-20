use std::io::stdout;

use crossterm::{
    terminal::{self,  EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Hide, Show},
    ExecutableCommand, 
    event::{self, Event, KeyCode},
};

use crate::components::{tty::PtyView, list::List};
use crate::components::element::Element;

pub struct Application {}

impl Application {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) -> std::io::Result<()> {
        let mut out_stream = stdout();
        let mut pty_view = PtyView::new();
        let mut options_list = List::new(
            vec![
                "Some option".to_string(),
                "Other option".to_string(),
                "Option 3".to_string(),
            ],
        );

        // Enter full-screen mode
        out_stream.execute(EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        out_stream.execute(Hide)?;

        loop {
            options_list.output(&mut out_stream)?;
            pty_view.output(&mut out_stream)?;

            if event::poll(std::time::Duration::from_millis(100))? {
                let event = event::read()?;
                options_list.on_event(&event)?;
                pty_view.on_event(&event)?;

                if let Event::Key(key_event) = event {
                    if key_event.code == KeyCode::Esc {
                        break;
                    }
                }
            }
        }

        out_stream
            .execute(Show)?
            .execute(LeaveAlternateScreen)?;

        terminal::disable_raw_mode()?;

        Ok(())
    }
}
