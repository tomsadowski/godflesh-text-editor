// pager/src/ui

use crate::util::{Rect, View};
use crate::tabs::{TabMgr};
use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyCode, KeyModifiers};
use std::io::{self, Write};

#[derive(Clone, Debug)]
pub struct UI {
    // bounds of this display
    rect: Rect,
    // view currently in use
    view: View,
    tabs: TabMgr,
    history: String,
    bookmarks: String,
} 
impl UI {
    // default view is View::Tab
    pub fn new(url: &str, w: u16, h: u16) -> Self {
        let rect = Rect::new(0, 0, w, h);
        Self {
            tabs: TabMgr::new(&rect, url),
            rect: rect,
            view: View::Tab,
            history: String::from(""),
            bookmarks: String::from(""),
        }
    }
    // display the current view
    pub fn view(&self, mut stdout: &std::io::Stdout) -> io::Result<()> {
        match &self.view {
            View::Tab => self.tabs.view(stdout),
            _ => Ok(()),
        }?;
        stdout.flush()
    }
    // resize all views, maybe do this in parallel?
    fn resize(&mut self, w: u16, h: u16) {
        self.rect = Rect::new(0, 0, w, h);
        self.tabs.resize(&self.rect);
    }
    // Resize and Control-C is handled here, 
    // otherwise delegate to current view
    pub fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Resize(w, h) => {
                self.resize(w, h); 
                true
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press, 
                ..
            }) => {
                self.view = View::Quit;
                true
            }
            Event::Key(KeyEvent {
                code: keycode, 
                kind: KeyEventKind::Press, 
                ..
            }) => 
                match &self.view {
                    View::Tab => self.tabs.update(&keycode),
                    _ => false,
                }
            _ => false,
        }
    }
    // no need to derive PartialEq for View
    pub fn quit(&self) -> bool {
        match self.view {
            View::Quit => true,
            _ => false,
        }
    }
} 
