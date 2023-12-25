use core::panic;

use crossterm::event::{KeyEvent, KeyCode, KeyEventKind};
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{
    as_24_bit_terminal_escaped,
    LinesWithEndings,
};

use super::element::Element;


pub struct Editor {
    body: String,
    cursor_x: u16,
    cursor_y: u16,
    ps: SyntaxSet,
    ts: ThemeSet,
    to_re_render: bool,
}


impl Editor {
    pub fn new() -> Self {
        Self { 
            body: String::new(),
            cursor_x: 0,
            cursor_y: 0,
            to_re_render: true,
            ps: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
        }
    }

    pub fn set_val(&mut self, val: &str) {
        self.body = String::from(val);
        self.to_re_render = true;
    }
}

impl Element for Editor {
    fn output(&mut self, console: &mut crate::console::Console, target: &mut super::block::Block) {
        target.reset();

        let syntax = self.ps.find_syntax_by_name("JSON").expect("No json extension");

        let mut h = HighlightLines::new(syntax, &self.ts.themes["InspiredGitHub"]);

        for line in LinesWithEndings::from(self.body.as_str()) {
            let ranges: Vec<(Style, &str)> = h.highlight_line(line, &self.ps).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
            target.write_str(console, escaped.as_str());
        }

        self.to_re_render = false;
    }

    fn on_event(&mut self, event: &crossterm::event::Event) -> std::io::Result<()> { 
        if let crossterm::event::Event::Key(KeyEvent { code, kind, .. }) = event {
            match code {
                KeyCode::Char(c) => {
                    if *kind == KeyEventKind::Press {
                        self.body.push(*c);
                        self.to_re_render = true;
                    }
                },
                KeyCode::Backspace | KeyCode::Delete => {
                    if *kind == KeyEventKind::Press {
                        self.body.pop();
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
