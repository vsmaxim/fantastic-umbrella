mod app;
mod components;
mod console;
mod layout;
mod model;

use app::Application;

fn main() -> std::io::Result<()> {
    let mut app = Application::new();
    app.run()
}

