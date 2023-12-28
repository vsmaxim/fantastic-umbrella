use crossterm::event::{KeyEvent, KeyCode, KeyEventKind};
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style, Color};
use crossterm::style::Color as CTColor;

use super::element::Element;


pub struct Editor {
    body: Vec<String>,
    cursor_l: usize,
    cursor_c: usize,
    ps: SyntaxSet,
    ts: ThemeSet,
    to_re_render: bool,
    only_cursor: bool,
    lines_changed: Vec<bool>,
    refresh: bool,
}


fn convert_color(st_color: Color) -> CTColor {
    CTColor::Rgb { r: st_color.r, g: st_color.g, b: st_color.b }
}


impl Editor {
    pub fn new() -> Self {
        Self { 
            body: vec![],
            lines_changed: vec![],
            cursor_l: 0,
            cursor_c: 0,
            to_re_render: true,
            only_cursor: false,
            ps: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
            refresh: true,
        }
    }

    pub fn set_val(&mut self, val: &str) {
        self.body = val.split("\n")
            .map(|part| part.to_string())
            .collect();

        self.lines_changed = [true].repeat(self.body.len());
        self.to_re_render = true;
    }
 
    fn safe_go_to(&mut self, line: usize, col: usize) {
        let line_cnt = self.body.len();

        self.cursor_l = if line_cnt > 0 {
            let safe_line = std::cmp::min(line_cnt - 1, line);
            let line_len = self.body[safe_line].len();

            self.cursor_c = if line_len > 0 {
                std::cmp::min(line_len, col)
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

    fn get_line_end(&self, line: usize) -> usize {
        self.body[line].len()
    }

    fn remove_line(&mut self, line: usize) {
        self.body.remove(line);
        self.lines_changed.remove(line);
        self.to_re_render = true;

        for line_ch in self.lines_changed.iter_mut().skip(line) {
            *line_ch = true;
        }
    }

    fn remove_line_interval(&mut self, line: usize, start: usize, end: usize) -> String {
        self.to_re_render = true;
        self.lines_changed[line] = true;

        let old_line = String::from(&self.body[line]);
        let mut new_line = String::from(&old_line[..start]);
        new_line.push_str(&old_line[end..]);
        self.body[line] = new_line;
        String::from(&old_line[start..end])
    }

    fn add_line(&mut self, line: usize, buf: &str) {
        self.to_re_render = true;

        self.body.insert(line, String::from(buf));
        self.lines_changed.insert(line, true);

        for line_ch in self.lines_changed.iter_mut().skip(line) {
            *line_ch = true;
        }

    }

    fn remove_char(&mut self, line: usize, col: usize) {
        self.to_re_render = true;
        self.lines_changed[line] = true;
        self.body[line].remove(col);
    }

    fn insert(&mut self, line: usize, col: usize, buf: &str) {
        self.body[line].insert_str(col, buf);
        self.to_re_render = true;
        self.lines_changed[line] = true;
    } 

    fn on_char_insert(&mut self, ch: &char) {
        if self.body.is_empty() {
            self.add_line(0, "");
            self.safe_go_to(0, 0);
        }

        self.insert(
            self.cursor_l,
            self.cursor_c,
            ch.to_string().as_str(),
        );

        self.safe_go_to_col(self.cursor_c + 1);
    }

    fn on_delete(&mut self) {
        if self.cursor_c == 0 { // Beginning of the string
            if self.cursor_l > 0 { // Line is not first
                let prev_line = self.cursor_l - 1;
                let cur_line = self.cursor_l;
                let prev_line_end = self.get_line_end(prev_line);
                let cur_text = self.body[cur_line].clone();
                self.insert(prev_line, prev_line_end, cur_text.as_str());
                self.remove_line(cur_line);
                self.safe_go_to(self.cursor_l - 1, prev_line_end);
            }
        } else { // Anywhere else
            self.safe_go_to_col(self.cursor_c - 1);
            self.remove_char(self.cursor_l, self.cursor_c);
        }
    }

    fn on_enter(&mut self) {
        if self.cursor_c == 0 { // Start of the line
            self.add_line(self.cursor_l, "");
            self.safe_go_to_line(self.cursor_l + 1);
        } else { 
            if self.cursor_c == self.get_line_end(self.cursor_l) { // End of the line 
                self.add_line(self.cursor_l + 1, "");
                self.safe_go_to(self.cursor_l + 1, 0);
            } else { // Middle of the line -> split 
                let substr = self.remove_line_interval(
                    self.cursor_l,
                    self.cursor_c,
                    self.get_line_end(self.cursor_l),
                );
                self.add_line(self.cursor_l + 1, &substr);
                self.safe_go_to(self.cursor_l + 1, 0);
            }
        }
    }

    fn on_move_up(&mut self) {
        if self.cursor_l > 0 {
            self.safe_go_to_line(self.cursor_l - 1);
            self.only_cursor = true;
            self.to_re_render = true;
        }
    }

    fn on_move_down(&mut self) {
        self.safe_go_to_line(self.cursor_l + 1);
        self.only_cursor = true;
        self.to_re_render = true;
    }

    fn on_move_left(&mut self) {
        if self.cursor_c > 0 {
            self.safe_go_to_col(self.cursor_c - 1);
            self.only_cursor = true;
            self.to_re_render = true;
        }
    }

    fn on_move_right(&mut self) {
        self.safe_go_to_col(self.cursor_c + 1);
        self.only_cursor = true;
        self.to_re_render = true;
    }
}

impl Element for Editor {
    fn output(&mut self, console: &mut crate::console::Console, target: &mut super::block::Block) {
        if self.to_re_render {
            console.hide_cursor();
            target.reset();

            if !self.only_cursor {
                let syntax = self.ps.find_syntax_by_name("JSON").expect("No json extension");
                let mut h = HighlightLines::new(syntax, &self.ts.themes["InspiredGitHub"]);

                for (line, render) in self.lines_changed.iter_mut().enumerate() {
                    if *render == true {
                        let ranges: Vec<(Style, &str)> = h.highlight_line(
                            &self.body[line],
                            &self.ps,
                        ).unwrap();

                        target.empty_line(console, line as u16);
                        target.move_to(console, 0, line as u16);
                        console.flush();

                        for (style, val) in ranges {
                            console.set_fg_color(convert_color(style.foreground));
                            console.set_bg_color(convert_color(style.background));
                            target.write(console, val.as_bytes());
                        }

                        target.write(console, "\n".as_bytes());
                        *render = false;
                    }

                    console.reset_color();
                    console.flush();
                }

                target.empty_after(console, self.body.len() as u16);
            }


            console.flush();
            console.show_cursor();

            target.move_to(
                console,
                self.cursor_c as u16,
                self.cursor_l as u16,
            );

            self.to_re_render = false;
            self.only_cursor = false;
        }
    }

    fn on_event(&mut self, event: &crossterm::event::Event) -> std::io::Result<()> { 
        if let crossterm::event::Event::Key(KeyEvent { 
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event {
            match code {
                KeyCode::Char(c) => self.on_char_insert(c),
                KeyCode::Backspace | KeyCode::Delete => self.on_delete(),
                KeyCode::Enter => self.on_enter(),
                KeyCode::Up => self.on_move_up(),
                KeyCode::Down => self.on_move_down(),
                KeyCode::Left => self.on_move_left(),
                KeyCode::Right => self.on_move_right(),
                _ => {},
            }
        }

        Ok(())
    }

    fn needs_re_render(&self) -> bool { 
        self.to_re_render
    }
}
