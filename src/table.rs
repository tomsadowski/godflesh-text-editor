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
pub struct ListBox {
    pub content: Vec<String>,
    pub pos: i8,
} impl ListBox {
    pub fn move_vertical(&mut self, n: i8) -> bool {
        let p: i8 = self.pos + n;
        if p < 0 || p > self.content.len().try_into().unwrap() {false}
        else {self.pos = p; true}
    } 
    pub fn get_url(self) {
    }
} impl Default for ListBox {
    fn default() -> Self {
        Self { content: vec!["listbox".to_string()], pos: 0 }
    }
} impl Widget for &ListBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
    }
}
