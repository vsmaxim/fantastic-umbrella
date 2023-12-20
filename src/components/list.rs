use std::io::Stdout;

use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::style::{Color, SetForegroundColor, SetBackgroundColor, Print, ResetColor};
use crossterm::execute;

use super::element::Element;


pub struct List {
    options: Vec<String>,
    selected: usize,
}

impl List {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            options,
            selected: 0,
        }
    }

    fn select_next(&mut self) {
        if self.selected < self.options.len() - 1 {
            self.selected += 1;
        }
    }

    fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
}

impl Element for List {
    fn output(&mut self, stdout: &mut Stdout) -> std::io::Result<()> { 
        for (i, option) in self.options.iter().enumerate() {
            let (fg_color, bg_color) = if i == self.selected {
                (Color::Black, Color::White)
            } else {
                (Color::White, Color::Black)
            };
            
            let output = option.trim().to_string();

            execute!(
                stdout,
                MoveTo(0, i as u16),
                SetForegroundColor(fg_color),
                SetBackgroundColor(bg_color),
                Print(output),
                Print("\n"),
                ResetColor,
            )?;
        }

        Ok(())
    }

    fn on_event(&mut self, event: &Event) -> std::io::Result<()> { 
        if let Event::Key(key_event) = event {
            if key_event.code == KeyCode::Up && key_event.kind == KeyEventKind::Press {
                self.select_prev();
            }

            if key_event.code == KeyCode::Down && key_event.kind == KeyEventKind::Press {
                self.select_next();
            }
        }

        Ok(())
    }
}

