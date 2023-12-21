use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{
    components::{
        list::{List, Request},
        element::Element,
        tty::PtyView,
        input::Input,
        shortcuts::ShortcutsView,
    },
    console::Console,
    layout::Layout,
};

pub struct Application {
}

impl Application {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut console = Console::new();
        let mut layout = Layout::new();

        let mut options_list = List::new(
            vec![
                Request {
                    method: "GET".into(),
                    title: "Get good".into(),
                },
                Request {
                    method: "POST".into(),
                    title: "Create a good".into(),
                },
                Request {
                    method: "PATCH".into(),
                    title: "Update good".into(),
                },
                Request {
                    method: "DELETE".into(),
                    title: "Delete good".into(),
                },
            ],
            layout.list_cont.width.into(),
        );

        let mut address_input = Input::new();
        let mut pty_view = PtyView::new(
            layout.req_cont.height,
            layout.req_cont.width,
        );

        let mut shortcuts_view = ShortcutsView::new();

        console.enter_full_screen();
        options_list.output(&mut console, &mut layout.list_cont);
        shortcuts_view.output(&mut console, &mut layout.hint_cont);

        loop {
            console.hide_cursor();
            console.flush();

            layout.render(&mut console);

            if shortcuts_view.needs_re_render() {
                shortcuts_view.output(&mut console, &mut layout.hint_cont);
            }

            if options_list.needs_re_render() {
                options_list.output(&mut console, &mut layout.list_cont);
            }

            if pty_view.needs_re_render() {
                pty_view.output(&mut console, &mut layout.req_cont);
            }

            if address_input.needs_re_render() {
                address_input.output(&mut console, &mut layout.input_cont);
            }

            if layout.req_cont.is_active() {
                layout.req_cont.reset_cursor(&mut console);
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
                    if layout.select_mode {
                        match code {
                            KeyCode::Esc => {
                                break;
                            },
                            _ => {
                                layout.navigate(&event, &mut console);
                            }
                        }
                    } else {
                        match code {
                            KeyCode::Esc => {
                                layout.enter_select_mode();
                            },
                            _ => {
                                if layout.list_cont.is_active() {
                                    options_list.on_event(&event)?;
                                } else if layout.req_cont.is_active() {  
                                    pty_view.on_event(&event)?;
                                } else if layout.input_cont.is_active() {
                                    address_input.on_event(&event)?;
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
