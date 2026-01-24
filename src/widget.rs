// widget

use crate::common::{
    Page, Bound, Screen, ScreenRange, DataRange, Pos, 
};
use crossterm::{
    QueueableCommand, 
    cursor::{MoveTo},
    style::{Print},
};
use std::{
    io::{self, Stdout, Write},
};

pub struct UI {
    pub edt: TextEditor,
    pub pos: Pos,
}

pub struct TextEditor {
    pub page: Page,
    pub text: Vec<String>,
}
impl TextEditor {
    pub fn new(screen: &Screen, spacer: u16, source: &str) -> Self {
        let src = source.lines().map(|s| String::from(s)).collect();
        Self {
            page: Page::new(screen, &src, spacer, 0),
            text: src,
        }
    }
    pub fn resize(&mut self, screen: &Screen, spacer: u16) {
        self.page = Page::new(screen, &self.text, spacer, 0);
    }
    pub fn view(&self, pos: Pos, mut stdout: &Stdout) -> io::Result<()> {
        let ranges = self.page.get_ranges(&pos);
        for (x, i, r) in ranges.into_iter() {
            stdout
                .queue(MoveTo(x, self.page.scr.y))?
                .queue(Print(&self.text[i][r.a..r.b]))?;
        }
        stdout.flush()
    }
    pub fn move_left(&self, mut pos: Pos, step: u16) -> bool {
        pos.move_left(&self.page, step)
    }
    pub fn move_right(&self, mut pos: Pos, step: u16) -> bool {
        pos.move_right(&self.page, step)
    }
    pub fn move_up(&self, mut pos: Pos, step: u16) -> bool {
        pos.move_up(&self.page, step)
    }
    pub fn move_down(&self, mut pos: Pos, step: u16) -> bool {
        pos.move_down(&self.page, step)
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
