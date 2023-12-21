use crossterm::event::{KeyEvent, KeyCode, KeyEventState, KeyEventKind};

use super::element::Element;


pub struct Input {
    value: String,
    to_empty: bool,
}

impl Input {
    pub fn new() -> Self {
        Self { 
            value: String::new(),
            to_empty: false,
        }
    }
}

impl Element for Input {
    fn output(&mut self, console: &mut crate::console::Console, target: &mut super::block::Block) {
        target.reset();

        if self.to_empty {
            self.to_empty = false;
            target.empty(console);
        }

        target.write(console, self.value.as_bytes());
    }

    fn on_event(&mut self, event: &crossterm::event::Event) -> std::io::Result<()> { 
        if let crossterm::event::Event::Key(KeyEvent { code, kind, .. }) = event {
            match code {
                KeyCode::Char(c) => {
                    if *kind == KeyEventKind::Press {
                        self.value.push(*c);
                    }
                },
                KeyCode::Backspace | KeyCode::Delete => {
                    if *kind == KeyEventKind::Press {
                        self.value.pop();
                        self.to_empty = true;
                    }
                },
                KeyCode::Enter => {
                },
                _ => {},
            }
        }

        Ok(())
    }
}
