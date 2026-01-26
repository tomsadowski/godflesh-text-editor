// ui

use crate::{
    bnd::{Page, Bound},
    scr::{Screen, ScreenRange, DataRange, Pos, PosCol},
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
        let scr = Screen::origin(w, h);
        let txt = std::fs::read_to_string(path).unwrap();
        let editor = TextEditor::new(&scr, 3, &txt);
        Self {
            view:   View::Text,
            pos:    Pos::origin(&editor.scr),
            editor: editor,
        }
    }
    // resize all views, maybe do this in parallel?
    fn resize(&mut self, w: u16, h: u16) {
        let scr = Screen::origin(w, h);
        self.pos = self.editor.resize(&scr, 3, &self.pos);
    }
    // display the current view
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .queue(Clear(ClearType::All))?
            .queue(cursor::Hide)?;

        self.editor.view(&self.pos, stdout)?;

        stdout
            .queue(MoveTo(self.pos.x, self.pos.y))?
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
                match self.editor.update(c, &self.pos) {
                    Some(p) => {
                        self.pos = p;
                        true
                    }
                    None => false,
                }
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
    pub scr: Screen,
}
impl TextEditor {
    pub fn new(scr: &Screen, spc: u16, source: &str) -> Self {
        let src: Vec<String> = source
            .lines()
            .map(|s| String::from(s))
            .collect();
        let tscr = scr.crop_east(10);
        let pscr = tscr.crop_north(3);
        Self {
            scr:    tscr,
            page:   Page::new(&pscr, &src, spc, spc),
            text:   src,
        }
    }
    pub fn view(&self, pos: &Pos, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .queue(MoveTo(self.scr.x, self.scr.y))?
            .queue(Print(format!("{:?}", pos)))?
            .queue(MoveTo(self.scr.x, self.scr.y + 1))?
            .queue(Print(format!("{:?}", self.page.x(&pos))))?;
        let ranges = self.page.get_ranges(&pos);
        for (y, i, r) in ranges.into_iter() {
            stdout
                .queue(MoveTo(self.page.scr.x, y))?
                .queue(Print(&self.text[i][r.start..r.end]))?
                .queue(MoveTo(self.page.scr.x().end + 2, y))?
                .queue(Print(format!("{} {}", r.start, r.end)))?;
        }
        stdout.flush()
    }
    pub fn resize(&mut self, scr: &Screen, spc: u16, pos: &Pos) -> Pos {
        let scr   = scr.crop_east(10);
        let pscr  = scr.crop_north(3);
        self.scr  = scr;
        self.page = Page::new(&pscr, &self.text, spc, spc);
        self.page.move_into_y(&self.page.move_into_x(pos))
    }
    pub fn update(&mut self, c: char, pos: &Pos) -> Option<Pos> {
        match c {
            'e' => {
                self.page.move_left(&pos, 1)
            }
            'n' => {
                self.page.move_right(&pos, 1)
            }
            'i' => {
                self.page.move_down(&pos, 1)
            }
            'o' => {
                self.page.move_up(&pos, 1)
            }
            _ => {
                None
            }
        }
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
