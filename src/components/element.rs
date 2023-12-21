use crossterm::event::Event;
use crate::console::Console;
use super::block::Block;


pub trait Element {
    fn output(&mut self, console: &mut Console, target: &mut Block);
    fn on_event(&mut self, event: &Event) -> std::io::Result<()>;
    fn needs_re_render(&self) -> bool;
}
