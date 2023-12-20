use std::io::Stdout;

use crossterm::event::Event;


pub trait Element {
    fn output(&mut self, stdout: &mut Stdout) -> std::io::Result<()>;
    fn on_event(&mut self, event: &Event) -> std::io::Result<()>;
}
