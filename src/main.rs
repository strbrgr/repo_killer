use app::App;
use ratatui::{TerminalOptions, Viewport};
use std::io;

#[path = "app.rs"]
mod app;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(1),
    });
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
