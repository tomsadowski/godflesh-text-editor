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
pub struct ViewBox {
    pub content: Vec<String>,
    pub x: i8, pub y: i8,
    pub width: i8,
} impl ViewBox {
    pub fn move_horizontal(&mut self, n: i8) -> bool {
        let p: i8 = self.x + n;
        if p < 0 || p > self.width {false}
        else {self.x = p; true}
    } 
    pub fn move_vertical(&mut self, n: i8) -> bool {
        let p: i8 = self.y + n;
        if p < 0 || p > self.content.len().try_into().unwrap() {false} 
        else {self.y = p; true}
    } 
    pub fn get_url(self) {
    }
} impl Default for ViewBox {
    fn default() -> Self {
        Self {
            content: vec!["viewbox".to_string()],
            x: 0, y: 0, width: 0,
        }
    }
} impl Widget for &ViewBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
    }
}
