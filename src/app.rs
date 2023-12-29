use std::sync::Arc;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{
    components::{
        element::Element,
        list::List,
        input::Input,
        shortcuts::ShortcutsView, editor::Editor,
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
    editor: Editor,
    shortcuts: ShortcutsView,
    current_request: Option<Request>,
    current_request_id: Option<usize>,
}

impl Application {
    pub fn new() -> Self {
        let layout = Layout::new();
        let model = Model::load_from_disk_or_default();

        Self {
            current_request: None,
            current_request_id: None,
            input: Input::new(),
            options: List::new(
                Arc::clone(&model.requests),
                layout.list_cont.width.into(),
            ),
            editor: Editor::new(),
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
            self.editor.set_val(&request.body);
            self.current_request = Some(request);
            self.current_request_id = Some(self.options.selected);
            self.layout.enter_select_mode();
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut console = Console::new();

        console.enter_full_screen();

        self.options.output(&mut console, &mut self.layout.list_cont);
        self.shortcuts.output(&mut console, &mut self.layout.hint_cont);
        self.layout.render(&mut console);

        loop {
            if self.shortcuts.needs_re_render() {
                self.shortcuts.output(&mut console, &mut self.layout.hint_cont);
            }

            if self.options.needs_re_render() {
                self.options.output(&mut console, &mut self.layout.list_cont);
            }

            if self.editor.needs_re_render() {
                self.editor.output(&mut console, &mut self.layout.req_cont);
            }

            if self.input.needs_re_render() {
                self.input.output(&mut console, &mut self.layout.input_cont);
            }

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
                            KeyCode::Char(c) => {

                                if c == 's' {
                                    self.current_request
                                        .as_ref()
                                        .map(|r| Request {
                                            title: r.title.to_string(),
                                            method: r.method.to_string(),
                                            url: self.input.get_value(),
                                            body: self.editor.get_body(),
                                        })
                                        .and_then(|r| {
                                            console.move_to(0, 50);
                                            console.write("saved on disk");
                                            console.flush();

                                            if let Some(req_id) = self.current_request_id {
                                                self.model.update_request(req_id, &r);
                                                Some(r)
                                            } else {
                                                None
                                            }
                                        });
                                }

                                if c == 'e' {
                                    if let Some(req) = &self.current_request {
                                        self.model.make_request(req);
                                    }
                                }
                            },
                            _ => {
                                self.layout.navigate(&event, &mut console);
                                console.hide_cursor()
                            }
                        }
                    } else {
                        match code {
                            KeyCode::Esc => {
                                self.layout.enter_select_mode();
                                console.hide_cursor();
                            },
                            _ => {
                                if self.layout.list_cont.is_active() {
                                    self.options.on_event(&event)?;
                                    self.check_option_selected();
                                } else if self.layout.req_cont.is_active() {  
                                    self.editor.on_event(&event)?;
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
