use crate::content::*;
use crate::table::*;
use crate::bar::*;
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
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Widget},
    crossterm::{
        ExecutableCommand,
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
    },
};

pub enum Message {
    Switch(View), 
    Move(Direction, i8), 
    Edit(KeyCode), 
    GoToUrl, 
    Stop,
}
#[derive(Clone, PartialEq)]
pub enum State {
    Running, 
    Stopped
}
#[derive(Clone, PartialEq)]
pub enum View {
    AddressBar, 
    History, 
    Bookmarks, 
    Content,
}
#[derive(Clone)]
pub struct Model {
    pub address_bar: TextBar,
    pub history: ListBox,
    pub bookmarks: ListBox,
    pub content: ViewBox,
    pub focus: View,
    pub state: State,
} impl Model {
    pub fn go_to_url(&mut self) {
    }
} impl Default for Model {
    fn default() -> Model {
        Self {
            state: State::Running,
            focus: View::Content,
            address_bar: TextBar::default(),
            content: ViewBox::default(),
            history: ListBox::default(),
            bookmarks: ListBox::default(),
        }
    }
} impl Widget for &Model {
    fn render(self, area: Rect, buf: &mut Buffer) {
    }
}
