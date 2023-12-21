use std::{io::{Stdout, stdout, Write}, fmt::Display};

use crossterm::{
    QueueableCommand,
    cursor::{MoveTo, Hide, Show},
    style::{Print, Color, SetForegroundColor, SetBackgroundColor, ResetColor},
    Command, ExecutableCommand, terminal::{EnterAlternateScreen, self, LeaveAlternateScreen, Clear}
};


pub struct Console {
    stdout: Stdout
}

impl Console {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    fn queue_safe<T: Command>(&mut self, cmd: T) {
        self.stdout.queue(cmd).expect("Couldn't queue");
    }

    pub fn enter_full_screen(&mut self) {
        self.stdout.execute(EnterAlternateScreen).unwrap();
        terminal::enable_raw_mode().unwrap();
        self.stdout.execute(Hide).unwrap();
    }

    pub fn exit_full_screen(&mut self) {
        self.stdout.execute(Show).unwrap();
        terminal::disable_raw_mode().unwrap();
        self.stdout.execute(LeaveAlternateScreen).unwrap();
    }

    pub fn set_fg_color(&mut self, fg: Color) {
        self.queue_safe(SetForegroundColor(fg));
    }

    pub fn set_bg_color(&mut self, bg: Color) {
        self.queue_safe(SetBackgroundColor(bg));
    }

    pub fn set_colors(&mut self, fg: Color, bg: Color) {
        self.set_fg_color(fg);
        self.set_bg_color(bg);
    }

    pub fn reset_color(&mut self) {
        self.queue_safe(ResetColor);
    }

    pub fn move_to(&mut self, x: u16, y: u16) {   
        self.queue_safe(MoveTo(x, y));
    }

    pub fn write<T: Display>(&mut self, v: T) {
        self.queue_safe(Print(v));
    }

    pub fn write_raw(&mut self, v: &[u8]) {
        self.stdout.write(v).unwrap();
    }

    pub fn hide_cursor(&mut self) {
        self.queue_safe(Hide);
    }

    pub fn show_cursor(&mut self) {
        self.queue_safe(Show);
    }

    pub fn flush(&mut self) {
        self.stdout.flush().expect("Couldn't flush");
    }
}

