use crossterm::event::{Event, KeyCode, KeyEventKind, KeyEvent};
use crossterm::style::Color;

use crate::console::Console;

use super::block::Block;
use super::element::Element;


pub struct Request {
    pub method: String,
    pub title: String,
}

pub struct List {
    values: Vec<Request>,
    selected: usize,
    max_method_len: usize,
    to_re_render: bool,
    width: usize,
}

impl List {
    pub fn new(options: Vec<Request>, width: usize) -> Self {
        Self {
            max_method_len: options
                .iter()
                .map(|v| v.method.len())
                .max()
                .unwrap_or(0),
            width: width,
            values: options,
            selected: 0,
            to_re_render: true,
        }
    }

    fn select_next(&mut self) {
        if self.selected < self.values.len() - 1 {
            self.selected += 1;
        }

        self.to_re_render = true
    }

    fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }

        self.to_re_render = true
    }
}

impl Element for List {
    fn output(
        &mut self,
        console: &mut Console,
        target: &mut Block,
    ) { 
        target.reset();

        for (i, option) in self.values.iter().enumerate() {
            let (fg_color, bg_color) = if i == self.selected {
                (Color::Black, Color::White)
            } else {
                (Color::Reset, Color::Reset)
            };
            
            let (method_fg_color, method_bg_color) = if i == self.selected {
                (Color::Black, Color::Green)
            } else {
                (Color::White, Color::Black)
            };

            let pad = self.max_method_len - option.method.len() + 2;
            let pad_left = std::cmp::max(1, pad / 2); 
            let pad_right = pad - pad_left;

            target.to_line_start(console);
            console.set_colors(method_bg_color, method_fg_color);

            target.write(console, " ".repeat(pad_left).as_bytes());
            target.write(console, option.method.to_uppercase().as_bytes());
            target.write(console, " ".repeat(pad_right).as_bytes());

            let pad_title_right = std::cmp::max(
                self.width - option.title.len() - pad_left - pad_right - option.method.len() - 2,
                0);

            console.set_colors(fg_color, bg_color);
            target.write(console, " ".as_bytes());
            target.write(console, option.title.as_bytes());
            target.write(console, " ".repeat(pad_title_right).as_bytes());
            console.reset_color();

            target.next_line(console);
        }

        self.to_re_render = false;
    }

    fn on_event(&mut self, event: &Event) -> std::io::Result<()> { 
        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event {
            match code {
                KeyCode::Up => self.select_prev(),
                KeyCode::Down => self.select_next(),
                _ => {}
            }
        }

        Ok(())
    }

    fn needs_re_render(&self) -> bool {  
        self.to_re_render
    }
}

