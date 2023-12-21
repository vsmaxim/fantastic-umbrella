mod app;
mod components;
mod console;

use app::Application;

fn main() -> std::io::Result<()> {
    let app = Application::new();
    app.run()
}

