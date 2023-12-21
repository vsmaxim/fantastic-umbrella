use crossterm::event::{KeyEvent, KeyCode, KeyEventKind};

use super::element::Element;


pub struct Input {
    value: String,
    to_empty: bool,
    to_re_render: bool,
}

impl Input {
    pub fn new() -> Self {
        Self { 
            value: String::new(),
            to_empty: false,
            to_re_render: true,
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
        self.to_re_render = true;
    }

    fn on_event(&mut self, event: &crossterm::event::Event) -> std::io::Result<()> { 
        if let crossterm::event::Event::Key(KeyEvent { code, kind, .. }) = event {
            match code {
                KeyCode::Char(c) => {
                    if *kind == KeyEventKind::Press {
                        self.value.push(*c);
                        self.to_re_render = true;
                    }
                },
                KeyCode::Backspace | KeyCode::Delete => {
                    if *kind == KeyEventKind::Press {
                        self.value.pop();
                        self.to_empty = true;
                        self.to_re_render = true;
                    }
                },
                KeyCode::Enter => {},
                _ => {},
            }
        }

        Ok(())
    }

    fn needs_re_render(&self) -> bool { 
        self.to_re_render
    }
}
