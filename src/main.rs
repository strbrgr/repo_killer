use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use logo::Logo;
use ratatui::{
    DefaultTerminal, Frame, TerminalOptions, Viewport, buffer::Buffer, layout::Rect, text::Line,
    widgets::Widget,
};
#[path = "widgets/logo.rs"]
mod logo;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(1),
    });
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    screen_type: ScreenType,
    counter: u8,
    exit: bool,
}

#[derive(Debug, Default)]
enum ScreenType {
    #[default]
    Welcome,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        // TODO: CTRL C should shut down on every screentype
        match &self.screen_type {
            ScreenType::Welcome => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL)
                    && key_event.code == KeyCode::Char('c')
                {
                    self.exit();
                }
            }
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.screen_type {
            ScreenType::Welcome => {
                Line::from("Welcome, hit enter to authenticate.").render(area, buf)
            }
        }
    }
}
