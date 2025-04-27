use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use input::{Input, InputMode};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, text::Line, widgets::Widget};
use std::io;

#[path = "widgets/input.rs"]
mod input;
#[path = "widgets/logo.rs"]
mod logo;

#[derive(Debug, Default)]
pub struct App {
    current_screen: CurrentScreen,
    exit: bool,
    token: Option<String>,
    waiting_for_token: bool,
    input_mode: InputMode,
    input_token: String,
}

#[derive(Debug, Default)]
enum CurrentScreen {
    #[default]
    Welcome,
    Auth,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Welcome,
            exit: false,
            token: None,
            waiting_for_token: false,
            input_mode: InputMode::Normal,
            input_token: String::new(),
        }
    }
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        match self.current_screen {
            CurrentScreen::Welcome => {
                frame.render_widget(self, frame.area());
            }
            CurrentScreen::Auth => {
                let input = Input::new();
                frame.render_widget(input, frame.area());
            }
        }
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
        if key_event.modifiers.contains(KeyModifiers::CONTROL)
            && key_event.code == KeyCode::Char('c')
        {
            self.exit();
        }

        match self.input_mode {
            InputMode::Normal => match self.current_screen {
                CurrentScreen::Welcome => {
                    if key_event.code == KeyCode::Enter {
                        self.waiting_for_token = true;
                        self.input_mode = InputMode::Editing;
                        self.current_screen = CurrentScreen::Auth;
                        let path = "https://github.com/settings/tokens/new?scopes=delete_repo,repo&description=Repo%20Remover%20Token";

                        match open::that(path) {
                            Ok(()) => {}
                            Err(err) => {
                                eprintln!("An error occurred when opening '{}': {}", path, err)
                            }
                        }
                    }
                }
                CurrentScreen::Auth => {}
            },
            InputMode::Editing => match key_event.code {
                KeyCode::Enter => {
                    self.token = Some(self.input_token.clone());
                    self.input_token.clear();
                    self.input_mode = InputMode::Normal;
                    self.waiting_for_token = false;
                    self.current_screen = CurrentScreen::Auth;
                }
                KeyCode::Char(c) => {
                    self.input_token.push(c);
                }
                KeyCode::Backspace => {
                    self.input_token.pop();
                }
                _ => {}
            },
        }

        match &self.current_screen {
            CurrentScreen::Welcome => {}
            CurrentScreen::Auth => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.current_screen {
            CurrentScreen::Welcome => {
                Line::from("Welcome, press enter to authenticate.").render(area, buf)
            }
            CurrentScreen::Auth => {
                Input::new().render(area, buf);
            }
        }
    }
}
