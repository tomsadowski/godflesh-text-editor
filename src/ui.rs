// ui

use crate::{
    scr::{self, Screen, ScreenRange, DataScreen, DataRange, Pos, PosCol},
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
}
impl UI {
    // start with View::Tab
    pub fn new(path: &str, w: u16, h: u16) -> Self {
        let scr = Screen::origin(w, h);
        let txt = std::fs::read_to_string(path).unwrap();
        let editor = TextEditor::new(&scr, 3, &txt);
        Self {
            view:   View::Text,
            editor: editor,
        }
    }
    // resize all views, maybe do this in parallel?
    fn resize(&mut self, w: u16, h: u16) {
        let scr = Screen::origin(w, h);
        self.editor.resize(&scr, 3);
    }
    // display the current view
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .queue(Clear(ClearType::All))?
            .queue(cursor::Hide)?;
        self.editor.view(stdout)

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
                self.editor.update(c) 
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
    pub text:   Vec<String>,
    pub lens:   Vec<usize>,
    pub dscr:   DataScreen,
    pub scr:    Screen,
    pub pos:    Pos,
}
impl TextEditor {
    pub fn new(scr: &Screen, spc: u16, source: &str) -> Self {
        let src: Vec<String> = source
            .lines()
            .map(|s| String::from(s))
            .collect();
        let lens: Vec<usize> = src
            .iter()
            .map(|s| s.len())
            .collect();
        let outer = scr.xcrop(8).ycrop(2);
        let dscr  = DataScreen::new(outer, spc, spc);
        let pos = dscr.new_pos();

        Self {
            scr:  scr.clone(),
            lens: lens,
            pos: pos,
            dscr: dscr,
            text:  src,
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        let ranges = scr::get_ranges(&self.dscr, &self.pos, &self.lens);
        stdout
            .queue(MoveTo(self.scr.x, self.scr.y))?
            .queue(Print(format!("{:?}", self.pos)))?;
        for (y, i, r) in ranges.into_iter() {
            stdout
                .queue(MoveTo(self.dscr.outer.x, y))?
                .queue(Print(&self.text[i][r.start..r.end]))?
                .queue(MoveTo(self.dscr.outer.x().end + 2, y))?
                .queue(Print(format!("{} {}", r.start, r.end)))?;
        }
        stdout
            .queue(MoveTo(self.pos.x, self.pos.y))?
            .queue(cursor::Show)?
            .flush()
    }
    pub fn resize(&mut self, scr: &Screen, spc: u16) {
        let outer = scr.xcrop(8).ycrop(2);
        let dscr  = DataScreen::new(outer, spc, spc);
        self.pos = dscr.new_pos();
        self.dscr = dscr;
        self.scr = scr.clone();
    }
    pub fn update(&mut self, c: char) -> bool {
        let o = match c {
            'e' => {
                scr::move_left(&self.dscr, &self.pos, 1)
            }
            'n' => {
                scr::move_right(&self.dscr, &self.pos, &self.lens, 1)
            }
            'i' => {
                scr::move_down(&self.dscr, &self.pos, &self.lens, 1)
            }
            'o' => {
                scr::move_up(&self.dscr, &self.pos, &self.lens, 1)
            }
            _ => {
                None
            }
        };
        match o {
            None => false,
            Some(p) => {
                self.pos = p;
                true
            }
        }
    }
}
