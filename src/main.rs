mod app;
mod components;

use app::Application;


fn main() -> std::io::Result<()> {
    let mut app = Application::new();
    app.run()?;
    Ok(())
}

