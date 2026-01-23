// widget

use crate::common::{
    Page, Bound, Screen, ScreenRange, DataRange, View, 
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
    pub editor: TextEditor,
    pub scr:    Screen,
}

pub struct TextEditor {
    pub page: Page,
    pub text: Vec<String>,
}
impl TextEditor {
    pub fn new(screen: &Screen, spacer: u16, source: &str) -> Self {
        let src = source.lines().map(|s| String::from(s)).collect();
        Self {
            page: Page::new(screen, &src, spacer, 0, ),
            text: src,
        }
    }
//  pub fn resize(&mut self, screen: &Screen, spacer: u16) {
//      self.bounds = ViewBound::new(screen.get_x(), spacer);
//  }
//  pub fn view(&self, view: View, mut stdout: &Stdout) -> io::Result<()> {
//      let DataRange(a, b) = self.page.get_y_data_range(&view);
//      let ViewBound(ScreenRange(start, _), _) = self.page.y_bound;
//      let ranges = self.page.get_x_data_range(&view);
//      for (i, l) in self.text[a..b].iter().enumerate() {
//          let DataRange(l1, l2) = ranges[i];
//          stdout
//              .queue(MoveTo(start + (i as u16), 0))?
//              .queue(Print(&l[l1..l2]))?;
//      }
//      stdout.flush()
//  }
//  pub fn move_left(&mut self, view: &View, step: u16) -> Option<View> {
//      self.page.move_backward(view, step)
//  }
//  pub fn move_right(&mut self, view: &View, step: u16) -> Option<View> {
//      self.page.move_forward(view, step)
//  }
//  pub fn delete(&mut self, view: &View) -> Option<View> {
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
