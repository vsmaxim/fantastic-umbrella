use std::{fmt, io};

use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::style::{Color, SetForegroundColor, SetBackgroundColor, Print, ResetColor};
use crossterm::execute;

use crate::console::Console;

use super::block::Block;
use super::element::Element;


pub struct List {
    options: Vec<String>,
    selected: usize,
}

impl List {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            options, selected: 0,
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
    fn output(
        &mut self,
        console: &mut Console,
        target: &mut Block,
    ) { 
        target.reset();

        for (i, option) in self.options.iter().enumerate() {
            let (fg_color, bg_color) = if i == self.selected {
                (Color::Black, Color::White)
            } else {
                (Color::White, Color::Black)
            };
            
            let output = option.trim().to_string();

            target.to_line_start(console);
            console.set_colors(fg_color, bg_color);
            target.write_str(console, output.as_str());
            console.reset_color();
            target.next_line(console);
        }
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

