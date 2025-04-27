use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crossterm::style::Stylize;

#[derive(Debug, Default)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

pub struct Input {
    pub input: String,
    pub character_index: usize,
    pub input_mode: InputMode, // Now it belongs to Input, not App
}

impl Input {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            character_index: 0,
            input_mode: InputMode::Editing,
        }
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        if self.character_index != 0 {
            let idx = self.byte_index();
            let mut chars: Vec<char> = self.input.chars().collect();
            chars.remove(self.character_index - 1);
            self.input = chars.into_iter().collect();
            self.move_cursor_left();
        }
    }

    fn move_cursor_left(&mut self) {
        self.character_index = self.character_index.saturating_sub(1);
    }

    fn move_cursor_right(&mut self) {
        self.character_index = (self.character_index + 1).min(self.input.chars().count());
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or_else(|| self.input.len())
    }
}

impl Widget for Input {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let input = Paragraph::new(self.input)
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(
                Block::default()
                    .title("Paste your token")
                    .borders(ratatui::widgets::Borders::ALL),
            );

        let vertical = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        input.render(vertical[1], buf);
    }
}
