// widget

use crate::common::{
    Page, Bound, Screen, ScreenRange, DataRange, Pos, 
};
use crossterm::{
    event::{Event, KeyEvent, KeyEventKind, KeyCode, KeyModifiers},
    QueueableCommand, 
    terminal::{Clear, ClearType},
    cursor::{self, MoveTo},
    style::{Print},
};
use std::{
    io::{self, Stdout, Write},
};

// view currently in use
#[derive(Debug, Clone)]
pub enum View {
    Text,
    Quit,
}
pub struct UI {
    pub view: View,
    pub editor: TextEditor,
    pub pos: Pos,
}
impl UI {
    // start with View::Tab
    pub fn new(path: &str, w: u16, h: u16) -> Self {
        let scr = Screen {x: 0, y: 0, w: w, h: h};
        let txt = std::fs::read_to_string(path).unwrap();
        Self {
            view: View::Text,
            editor: TextEditor::new(&scr, 0, &txt),
            pos: Pos {x: 0, y: 0, i: 0, j: 0},
        }
    }
    // resize all views, maybe do this in parallel?
    fn resize(&mut self, w: u16, h: u16) {
        let scr = Screen {x: 0, y: 0, w: w, h: h};
        self.editor.resize(&scr, 0);
    }
    // display the current view
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .queue(Clear(ClearType::All))?
            .queue(cursor::Hide)?;
        self.editor.view(&self.pos, stdout)?;
        stdout
            .queue(cursor::Show)?
            .flush()
    }
    // Resize and Control-C is handled here, 
    // otherwise delegate to current view
    pub fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Resize(w, h) => {
                self.resize(w, h); 
                true
            }
            Event::Key(
                KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code:      KeyCode::Char('c'),
                    kind:      KeyEventKind::Press, ..
                }
            ) => {
                self.view = View::Quit;
                true
            }
            Event::Key(
                KeyEvent {
                    code: KeyCode::Char(c), 
                    kind: KeyEventKind::Press, ..
                }
            ) => {
                self.editor.update(c, &mut self.pos)
            }
            _ => false,
        }
    }
    // no need to derive PartialEq for View
    pub fn is_quit(&self) -> bool {
        match self.view {
            View::Quit => 
                true, 
            _ => 
                false
        }
    }
} 

pub struct TextEditor {
    pub page: Page,
    pub text: Vec<String>,
}
impl TextEditor {
    pub fn new(screen: &Screen, spacer: u16, source: &str) -> Self {
        let src: Vec<String> = source
            .lines()
            .map(|s| String::from(s))
            .collect();
        Self {
            page: Page::new(screen, &src, spacer, 0),
            text: src,
        }
    }
    pub fn resize(&mut self, scr: &Screen, spc: u16) {
        self.page = Page::new(scr, &self.text, spc, 0);
    }
    pub fn update(&mut self, c: char, pos: &mut Pos) -> bool {
        match c {
            'e' => {
                pos.move_left(&self.page, 1)
            }
            'i' => {
                pos.move_down(&self.page, 1)
            }
            'o' => {
                pos.move_up(&self.page, 1)
            }
            'n' => {
                pos.move_right(&self.page, 1)
            }
            _ => {
                false
            }
        }
    }
    pub fn view(&self, pos: &Pos, mut stdout: &Stdout) -> io::Result<()> {
        let ranges = self.page.get_ranges(&pos);
        for (y, i, r) in ranges.into_iter() {
            stdout
                .queue(MoveTo(self.page.scr.x, y))?
                .queue(Print(&self.text[i][r.a..r.b]))?;
        }
        stdout.flush()
    }
//  pub fn delete(&mut self, pos: &View) -> Option<View> {
//      let View(cursor, _) = view;
//      if self.bounds.screen_range.get_idx(view) == self.text.len() + 1 {
//          return None
//      }
//      let mut view = view.clone();
//      self.text.remove(self.bounds.screen_range.get_idx(&view));
//      if cursor + 1 != self.bounds.screen_range.1 {
//          if let Some(s) = view.move_forward(&self.bounds, self.text.len() + 1, 1) {
//              view = s;
//          }
//      }
//      Some(view)
//  }
//  pub fn backspace(&mut self, col: &View) -> Option<View> {
//      let cursor_range = &self.bounds.screen_range;
//      if col.is_start(&self.bounds) {
//          return None
//      } 
//      match col.move_backward(&self.bounds, 1) {
//          true => {
//              self.text.remove(cursor_range.get_idx(&col));
//      //      self.cursor.resize(0, self.text.len(), &self.range);
//              if col.cursor + 1 != cursor_range.1 {
//                  col.move_forward(&self.bounds, 1);
//                  Some(col.clone())
//              } else {
//                  Some(col.clone())
//              }
//          }
//          false => None,
//      }
//  }
//  pub fn insert(&mut self, view: &View, c: char) -> Option<View>{
//      if self.bounds.screen_range.get_idx(view) + 1 == self.text.len() {
//          self.text.push(c);
//      } else {
//          self.text.insert(self.bounds.screen_range.get_idx(view), c);
//      }
//      view.move_forward(&self.bounds, self.text.len() + 1, 1)
//  }
}
