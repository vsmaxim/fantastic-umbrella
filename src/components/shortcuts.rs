use super::element::Element;

pub struct ShortcutsView {
    to_re_render: bool,
}

impl ShortcutsView {
    pub fn new() -> Self {
        return Self {
            to_re_render: true,
        }
    }
}

impl Element for ShortcutsView { 
    fn output(&mut self, console: &mut crate::console::Console, target: &mut super::block::Block) { 
        console.reset_color();
        target.reset();

        target.write(console, "[s] Send".as_bytes());
        target.write(console, " [e] Execute".as_bytes());

        console.flush();
        self.to_re_render = false;
    }

    fn on_event(&mut self, event: &crossterm::event::Event) -> std::io::Result<()> {
       Ok(()) 
    }

    fn needs_re_render(&self) -> bool { 
        self.to_re_render
    }
}


