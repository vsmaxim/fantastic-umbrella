use std::sync::Arc;

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, style::Stylize};

use crate::{
    components::{
        element::Element,
        list::List,
        tty::PtyView,
        input::Input,
        shortcuts::ShortcutsView,
    },
    console::Console,
    layout::Layout,
    model::{Model, Request},
};

pub struct Application {
    model: Model,
    layout: Layout,
    input: Input,
    options: List,
    pty: PtyView,
    shortcuts: ShortcutsView,
    current_request: Option<Request>,
}

impl Application {
    pub fn new() -> Self {
        let layout = Layout::new();
        let model = Model::load_from_disk_or_default();

        Self {
            current_request: None,
            input: Input::new(),
            options: List::new(
                Arc::clone(&model.requests),
                layout.list_cont.width.into(),
            ),
            pty: PtyView::new(
                layout.req_cont.height,
                layout.req_cont.width,
            ),
            shortcuts: ShortcutsView::new(),
            model,
            layout,
        }
    }

    pub fn check_option_selected(&mut self) {
        if self.options.option_selected {
            let request_ptr = self.model.requests.read().unwrap()[self.options.selected].clone();
            let request = Request::from(request_ptr.as_ref());
            self.options.option_selected = false;
            self.input.set_val(&request.url);
            self.current_request = Some(request);
            self.layout.enter_select_mode();
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut console = Console::new();

        console.enter_full_screen();
        self.options.output(&mut console, &mut self.layout.list_cont);
        self.shortcuts.output(&mut console, &mut self.layout.hint_cont);

        loop {
            console.hide_cursor();
            console.flush();

            self.layout.render(&mut console);

            if self.shortcuts.needs_re_render() {
                self.shortcuts.output(&mut console, &mut self.layout.hint_cont);
            }

            if self.options.needs_re_render() {
                self.options.output(&mut console, &mut self.layout.list_cont);
            }

            if self.pty.needs_re_render() {
                self.pty.output(&mut console, &mut self.layout.req_cont);
            }

            if self.input.needs_re_render() {
                self.input.output(&mut console, &mut self.layout.input_cont);
            }

            if self.layout.req_cont.is_active() {
                self.layout.req_cont.reset_cursor(&mut console);
                console.show_cursor();
            } 

            console.flush();

            if event::poll(std::time::Duration::from_millis(10))? {
                let event = event::read()?;

                if let Event::Key(KeyEvent { 
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) = event {
                    if self.layout.select_mode {
                        match code {
                            KeyCode::Esc => {
                                break;
                            },
                            _ => {
                                self.layout.navigate(&event, &mut console);
                            }
                        }
                    } else {
                        match code {
                            KeyCode::Esc => {
                                self.layout.enter_select_mode();
                            },
                            _ => {
                                if self.layout.list_cont.is_active() {
                                    self.options.on_event(&event)?;
                                    self.check_option_selected();
                                } else if self.layout.req_cont.is_active() {  
                                    self.pty.on_event(&event)?;
                                } else if self.layout.input_cont.is_active() {
                                    self.input.on_event(&event)?;
                                }
                            }
                        };
                    }
                }
            }
        }

        console.exit_full_screen();
        Ok(())
    }
}
