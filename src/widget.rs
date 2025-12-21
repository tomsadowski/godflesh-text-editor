// pager/src/widget

use crate::util::{self, Rect, ScrollingCursor, GetColors};
use crossterm::{QueueableCommand, cursor, style};
use crossterm::event::{KeyCode};
use std::io::{self, Stdout};

#[derive(Clone, Debug)]
pub enum DialogMsg {
    None,
    Cancel,
    Submit,
}
#[derive(Clone, Debug)]
pub enum InputType {
    None,
    Choose((char, Vec<(char, String)>)),
    Input(String),
}
#[derive(Clone, Debug)]
pub struct Dialog<T> {
    rect: Rect,
    prompt: String,
    pub action: T,
    pub input: InputType,
}
impl<T: Clone + std::fmt::Debug> Dialog<T> {
    pub fn new(rect: &Rect, action: T, input: InputType, prompt: &str) -> Self 
    {
        Self {
            rect: rect.clone(),
            action: action,
            input: input,
            prompt: String::from(prompt), 
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .queue(cursor::MoveTo(self.rect.x + 2, self.rect.y + 2))?
            .queue(style::Print(self.prompt.as_str()))?
            .queue(cursor::MoveTo(self.rect.x + 2, self.rect.y + 4))?
            .queue(style::Print(format!("{:?}", self.input)))?;
        Ok(())
    }
    // No wrapping yet, so resize is straightforward
    pub fn resize(&mut self, rect: &Rect) {
        self.rect = rect.clone();
    }
    // Keycode has various meanings depending on the InputType.
    // The match statement might be moved to impl InputType
    pub fn update(&mut self, keycode: &KeyCode) -> Option<DialogMsg> {
        match (&mut self.input, keycode) {
            // Pressing Escape always cancels
            (_, KeyCode::Esc) => {
                Some(DialogMsg::Cancel)
            }
            // Pressing Enter in a choosebox means nothing
            (InputType::Choose(_), KeyCode::Enter) => {
                Some(DialogMsg::None)
            }
            // Otherwise, pressing Enter means Submit
            (_, KeyCode::Enter) => {
                Some(DialogMsg::Submit)
            }
            // Backspace works in inputbox
            (InputType::Input(v), KeyCode::Backspace) => {
                v.pop();
                Some(DialogMsg::None)
            }
            // Typing works in inputbox
            (InputType::Input(v), KeyCode::Char(c)) => {
                v.push(*c);
                Some(DialogMsg::None)
            }
            // Check for meaning in choosebox
            (InputType::Choose(t), KeyCode::Char(c)) => {
                let chars: Vec<char> = t.1.iter().map(|e| e.0).collect();
                match chars.contains(&c) {
                    true => {
                        t.0 = *c;
                        Some(DialogMsg::Submit)
                    }
                    false => None,

                }
            }
            _ => None,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Selector<T> {
    rect: Rect,
    source: Vec<(T, String)>,
    wrap: bool,
    display: Vec<(usize, String)>,
    pub cursor: ScrollingCursor,
} 
impl<T: Clone + GetColors> Selector<T> {
    pub fn new(rect: &Rect, source: Vec<(T, String)>, wrap: bool) -> Self {
        let display = match wrap {
            true => util::wraplist(&source, rect.w),
            false => util::cutlist(&source, rect.w),
        };
        return Self {
            rect: rect.clone(),
            wrap: wrap,
            source: source,
            cursor: ScrollingCursor::new(display.len(), &rect),
            display: display,
        }
    }
    pub fn resize(&mut self, rect: &Rect) {
        self.rect = rect.clone();
        self.display = match self.wrap {
            true => util::wraplist(&self.source, rect.w),
            false => util::cutlist(&self.source, rect.w),
        };
        self.cursor.resize(self.display.len(), rect);
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        let (a, b) = self.cursor.slicebounds();
        for (j, (i, text)) in self.display[a..b].iter().enumerate() {
            stdout
                .queue(cursor::MoveTo(self.rect.x, self.rect.y + j as u16))?
                .queue(style::SetColors(self.source[*i].0.getcolors()))?
                .queue(style::Print(text.as_str()))?;
        }
        stdout.queue(cursor::MoveTo(0, self.cursor.cursor))?;
        Ok(())
    }
    pub fn selectundercursor(&self) -> &T {
        &self.source[self.display[self.cursor.index()].0].0
    }
} 
