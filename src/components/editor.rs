use crossterm::event::{KeyEvent, KeyCode, KeyEventKind};
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::as_24_bit_terminal_escaped;

use super::element::Element;


pub struct Editor {
    body: Vec<String>,
    cursor_l: usize,
    cursor_c: usize,
    ps: SyntaxSet,
    ts: ThemeSet,
    to_re_render: bool,
    only_cursor: bool,
}


impl Editor {
    pub fn new() -> Self {
        Self { 
            body: vec![String::new()],
            cursor_l: 0,
            cursor_c: 0,
            to_re_render: true,
            only_cursor: false,
            ps: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
        }
    }

    pub fn set_val(&mut self, val: &str) {
        self.body = val.split("\n")
            .map(|part| part.to_string())
            .collect();

        self.to_re_render = true;
    }
 
    fn safe_go_to(&mut self, line: usize, col: usize) {
        let line_cnt = self.body.len();

        self.cursor_l = if line_cnt > 0 {
            let safe_line = std::cmp::min(line_cnt - 1, line);
            let line_len = self.body[safe_line].len();

            self.cursor_c = if line_len > 0 {
                std::cmp::min(line_len - 1, col)
            } else {
                0
            };

            safe_line
        } else {
            0
        };
    }

    fn safe_go_to_col(&mut self, col: usize) {
        self.safe_go_to(self.cursor_l, col);
    }

    fn safe_go_to_line(&mut self, line: usize) {
        self.safe_go_to(line, self.cursor_c);
    }
}

impl Element for Editor {
    fn output(&mut self, console: &mut crate::console::Console, target: &mut super::block::Block) {

        if self.to_re_render {
            if !self.only_cursor {
                console.hide_cursor();
                target.empty(console);
                target.reset();

                let syntax = self.ps.find_syntax_by_name("JSON").expect("No json extension");
                let mut h = HighlightLines::new(syntax, &self.ts.themes["InspiredGitHub"]);

                for line in &self.body {
                    let ranges: Vec<(Style, &str)> = h.highlight_line(line.as_str(), &self.ps).unwrap();
                    let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                    target.write_str(console, escaped.as_str());
                    target.write(console, "\n".as_bytes());
                }

                target.reset();
                console.show_cursor();
                console.flush();
            }

            target.move_to(
                console,
                self.cursor_c as u16,
                (self.cursor_l + 1) as u16,
            );
        }


        self.to_re_render = false;
        self.only_cursor = false;
    }

    fn on_event(&mut self, event: &crossterm::event::Event) -> std::io::Result<()> { 
        if let crossterm::event::Event::Key(KeyEvent { 
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event {
            match code {
                KeyCode::Char(c) => {
                    if self.body.is_empty() {
                        self.body.push(String::new());
                        self.safe_go_to(0, 0);
                    }

                    self.body[self.cursor_l].insert(self.cursor_c, *c);
                    self.safe_go_to_col(self.cursor_c + 1);
                    self.to_re_render = true;
                },
                KeyCode::Backspace | KeyCode::Delete => {
                    if self.cursor_c == 0 { // Beginning of the string
                        if self.cursor_l > 0 { // Line is not first
                            let prev_line = self.cursor_l - 1;
                            let cur_line = self.cursor_l;
                            let cur_text = self.body[cur_line].clone();
                            self.body[prev_line].push_str(cur_text.as_str());
                            self.body.remove(cur_line);
                            self.safe_go_to_line(self.cursor_l - 1);
                            self.to_re_render = true;
                        }
                    } else { // Anywhere else
                        self.safe_go_to_col(self.cursor_c - 1);
                        self.body[self.cursor_l].remove(self.cursor_c);
                        self.to_re_render = true;
                    }
                },
                KeyCode::Enter => {
                    if self.cursor_c == 0 { // Start of the line
                        self.body.insert(self.cursor_l, String::new());
                        self.safe_go_to_line(self.cursor_l + 1);
                        self.to_re_render = true;
                    } else { // End of the line 
                        let line_length = self.body[self.cursor_l].len();

                        if line_length == 0 || self.cursor_c == line_length - 1{
                            self.body.insert(self.cursor_l + 1, String::new());
                            self.safe_go_to(self.cursor_l + 1, 0);
                            self.to_re_render = true;
                        } else { // Middle of the line -> split 
                            let substr = String::from(&self.body[self.cursor_l][self.cursor_c..]);
                            self.body[self.cursor_l].truncate(self.cursor_c);
                            self.body.insert(self.cursor_l + 1, substr);
                            self.safe_go_to(self.cursor_l + 1, 0);
                            self.to_re_render = true;
                        }
                    }
                },
                KeyCode::Up => {
                    if self.cursor_l > 0 {
                        self.safe_go_to_line(self.cursor_l - 1);
                        self.only_cursor = true;
                        self.to_re_render = true;
                    }
                },
                KeyCode::Down => {
                    self.safe_go_to_line(self.cursor_l + 1);
                    self.only_cursor = true;
                    self.to_re_render = true;
                },
                KeyCode::Left => {
                    if self.cursor_c > 0 {
                        self.safe_go_to_col(self.cursor_c - 1);
                        self.only_cursor = true;
                        self.to_re_render = true;
                    }
                },
                KeyCode::Right => {
                    self.safe_go_to_col(self.cursor_c + 1);
                    self.only_cursor = true;
                    self.to_re_render = true;
                },
                _ => {},
            }
        }

        Ok(())
    }

    fn needs_re_render(&self) -> bool { 
        self.to_re_render
    }
}
