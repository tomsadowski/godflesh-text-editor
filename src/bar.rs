use crossterm::{
    execute, cursor, 
    event::{self, Event, KeyModifiers, KeyEventKind, KeyCode, KeyEvent},
    terminal::{self, ScrollUp, SetSize, size}
};
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend}, buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span, Text},
    widgets::{Axis, Block, Borders, Paragraph, Widget},
};

#[derive(Clone)]
pub struct TextBar {
    pub content: String,
    pub pos: i8,
    pub width: i8,
} impl TextBar {
    pub fn move_horizontal(&mut self, n: i8) -> bool {
        let p: i8 = self.pos + n;
        if p < 0 || p > self.width {false}
        else {self.pos = p; true}
    } 
    pub fn edit(&mut self, keycode: KeyCode) {
    }
    pub fn get_url(self) {
    }
} impl Default for TextBar {
    fn default() -> Self {
        Self { pos: 0, width: 20, content: "textbar".to_string() }
    }
} impl Widget for &TextBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
    }
}
