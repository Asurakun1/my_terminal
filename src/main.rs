use std::error::Error;

use my_terminal::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let terminal = ratatui::init();
    let mut app = App::new(terminal);
    app.run()?;

    ratatui::restore();

    Ok(())
}
