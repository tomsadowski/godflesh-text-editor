// ui

use crate::{
    scr::{self, Screen, DataScreen, Pos},
};
use crossterm::{
    QueueableCommand, 
    event::{Event, KeyEvent, KeyEventKind, KeyCode, KeyModifiers},
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
    pub txt:    Vec<String>,
    pub txtlen: Vec<usize>,
    pub txtscr: DataScreen,
    pub scr:    Screen,
    pub pos:    Pos,
}
impl TextEditor {
    pub fn new(scr: &Screen, spc: u16, txt: &str) -> Self {
        let txt: Vec<String> = txt
            .lines()
            .map(|s| String::from(s))
            .collect();
        let txtlen: Vec<usize> = txt
            .iter()
            .map(|s| s.len())
            .collect();
        let outer   = scr.crop_x(8).crop_y(2);
        let txtscr  = DataScreen::new(outer, spc, spc);
        let pos     = txtscr.new_pos();

        Self {
            scr:    scr.clone(),
            txtscr: txtscr,
            pos:    pos,
            txtlen: txtlen,
            txt:    txt,
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .queue(MoveTo(self.scr.x, self.scr.y))?
            .queue(Print(format!("{:?}", self.pos)))?;

        let ranges = scr::get_ranges(&self.txtscr, &self.pos, &self.txtlen);
        let screen_start = self.txtscr.outer.x;
        let screen_end   = self.txtscr.outer.x().end;

        for (screen_idx, data_idx, start, end) in ranges.into_iter() {
            stdout
                .queue(MoveTo(screen_start, screen_idx))?
                .queue(Print(&self.txt[data_idx][start..end]))?
                .queue(MoveTo(screen_end + 2, screen_idx))?
                .queue(Print(format!("{} {}", start, end)))?;
        }

        stdout
            .queue(MoveTo(self.pos.x, self.pos.y))?
            .queue(cursor::Show)?
            .flush()
    }
    pub fn resize(&mut self, scr: &Screen, spc: u16) {
        self.scr    = scr.clone();
        let outer   = self.scr.crop_x(8).crop_y(2);
        self.txtscr = DataScreen::new(outer, spc, spc);
        self.pos    = scr::resize(&self.txtscr, &self.pos, &self.txtlen);
    }
    pub fn update(&mut self, c: char) -> bool {
        let o = match c {
            'e' => {
                scr::move_left(&self.txtscr, &self.pos, 1)
            }
            'n' => {
                scr::move_right(&self.txtscr, &self.pos, &self.txtlen, 1)
            }
            'i' => {
                scr::move_down(&self.txtscr, &self.pos, &self.txtlen, 1)
            }
            'o' => {
                scr::move_up(&self.txtscr, &self.pos, &self.txtlen, 1)
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
