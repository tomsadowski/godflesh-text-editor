// widget

use crate::common;
use crossterm::{
    QueueableCommand, cursor,
    terminal::{self, ClearType},
    style::{self, Color},
};
use std::{
    io::{self, Stdout, Write},
};

// a rectangle specified by a point and some lengths
#[derive(Clone, Debug)]
pub struct Rect {
    pub x: u16, 
    pub y: u16, 
    pub w: u16, 
    pub h: u16,
}
impl Rect {
    pub fn origin(w: u16, h: u16) -> Self {
        Self {x: 0, y: 0, w: w, h: h}
    }
    pub fn mid_x(&self) -> u16 {
        self.x + (self.w / 2)
    }
    pub fn mid_y(&self) -> u16 {
        self.y + (self.h / 2)
    }
    pub fn quarter_x(&self) -> Result<Range, String> {
        let start = self.x + (self.w / 4);
        let end   = self.x + (self.w - (self.w / 4));
        Range::new(usize::from(start), usize::from(end))
    }
    pub fn quarter_y(&self) -> Result<Range, String> {
        let start = self.y + (self.h / 4);
        let end   = self.y + (self.h - (self.h / 4));
        Range::new(usize::from(start), usize::from(end))
    }
    pub fn horizontal(&self) -> Result<Range, String> {
        let start = self.x;
        let end   = self.x + self.w;
        Range::new(usize::from(start), usize::from(end))
    }
    pub fn verticle(&self) -> Result<Range, String> {
        let start = self.y;
        let end   = self.y + self.h;
        Range::new(usize::from(start), usize::from(end))
    }
}
#[derive(Clone, Debug)]
pub struct Range {
    a: usize,
    b: usize,
}
impl Range {
    pub fn new(a: usize, b: usize) -> Result<Self, String> {
        match a > b {
            true => 
                Err(format!("invalid range: (a: {}) > (b: {})", a, b)),
            false => 
                Ok(Self {a: a, b: b}),
        }
    }
    pub fn len(&self) -> usize {
        self.b - self.a
    }
    pub fn start(&self) -> usize {
        self.a
    }
    pub fn end(&self) -> usize {
        self.b
    }
}
#[derive(Clone, Debug)]
pub struct Cursor {
    tip:       usize,
    inner:     Range,
    outer:     Range,
    buf:       usize,
    cursor:    usize,
    scroll:    usize,
    maxscroll: usize,
}
impl Cursor {
    // private helper returning (outer, inner) ranges
    fn get_ranges(len: usize, r: &Range, buf: usize) -> (Range, Range) {
        if len < r.len() {
            let range = Range::new(r.start(), r.start() + len).unwrap();
            return (range.clone(), range)
        } else {
            // if buf is too big then return input
            let inner = 
                match Range::new(r.start() + buf, r.end() - (buf + 1)) {
                    Ok(range) => range,
                    _         => r.clone(),
                };
            return (r.clone(), inner)
        }
    }
    pub fn new(len: usize, r: &Range, buf: u8) -> Self {
        let buf = usize::from(buf);
        let tip = 0;
        let len = len + tip;
        let (outer, inner) = Self::get_ranges(len, r, buf);
        Self {
            tip:       tip,
            buf:       buf,
            scroll:    0, 
            maxscroll: len - outer.len(),
            cursor:    outer.start(), 
            outer:     outer,
            inner:     inner,
        }
    }
    pub fn text(len: usize, r: &Range) -> Self {
        let buf = 0;
        let tip = 1;
        let len = len + tip;
        let (outer, inner) = Self::get_ranges(len, r, buf);
        Self {
            tip:       tip,
            buf:       buf,
            scroll:    0, 
            maxscroll: len - outer.len(),
            cursor:    outer.start(), 
            outer:     outer,
            inner:     inner,
        }
    }
    pub fn resize(&mut self, len: usize, r: &Range) {
        let len = len + self.tip;
        let (outer, inner) = Self::get_ranges(len, r, self.buf);
        self.outer     = outer;
        self.inner     = inner;
        self.maxscroll = len - self.outer.len();
        self.scroll    = std::cmp::min(self.scroll, self.maxscroll);
        self.cursor    = std::cmp::min(self.cursor, self.inner.end());
    }
    pub fn backward(&mut self, mut step: usize) -> bool {
        // no scroll change
        if self.scroll == usize::MIN {
            // no cursor change. return false
            if self.cursor == self.outer.start() {
                return false
            // some cursor change
            } else if self.outer.start() + step <= self.cursor {
                self.cursor -= step; 
            } else {
                self.cursor = self.outer.start();
            }
        // change cursor only
        } else if (self.inner.start() + step) <= self.cursor {
            self.cursor -= step; 
        // change scroll and possibly cursor
        } else {
            // subtract from step the distance between cursor and innerstart
            step -= self.cursor - self.inner.start();
            // move cursor to innerstart
            self.cursor = self.inner.start();
            // change scroll only
            if step <= self.scroll {
                self.scroll -= step;
            // change scroll and cursor
            } else {
                step -= self.scroll;
                self.scroll = usize::MIN;
                if self.outer.start() + step <= self.cursor {
                    self.cursor -= step; 
                } else {
                    self.cursor = self.outer.start();
                }
            }
        }
        return true
    }
    pub fn forward(&mut self, mut step: usize) -> bool {
        // no scroll change
        if self.scroll == self.maxscroll {
            // no cursor change. return false
            if self.cursor == self.outer.end() - 1 {
                return false
            // some cursor change
            } else if self.cursor + step <= self.outer.end() - 1 {
                self.cursor += step;
            } else {
                self.cursor = self.outer.end() - 1;
            }
        // change cursor only
        } else if (self.cursor + step) <= self.inner.end() {
            self.cursor += step;
        // change scroll and possibly cursor
        } else {
            // subtract from step the distance between cursor and innerend
            step -= self.inner.end() - self.cursor;
            // move cursor to innerend
            self.cursor = self.inner.end();
            // change scroll only
            if self.scroll + step <= self.maxscroll {
                self.scroll += step;
            // change scroll and cursor
            } else {
                step -= self.maxscroll - self.scroll;
                self.scroll = self.maxscroll;
                if self.cursor + step <= self.outer.end() - 1 {
                    self.cursor += step;
                } else {
                    self.cursor = self.outer.end() - 1;
                }
            }
        }
        return true
    }
    pub fn is_start(&self) -> bool {
        self.scroll == usize::MIN &&
        self.cursor == self.outer.start() 
    }
    pub fn is_end(&self) -> bool {
        self.scroll == self.maxscroll &&
        self.cursor == self.outer.end() 
    }
    // return scroll
    pub fn get_scroll(&self) -> usize {
        self.scroll
    }
    // index of cursor within its range
    pub fn get_index(&self) -> usize {
        self.scroll + self.cursor - self.outer.start()
    }
    // return u16 for cursor
    pub fn get_cursor(&self) -> u16 {
        u16::try_from(self.cursor).unwrap()
    }
    // return u16 for outer.start
    pub fn get_screen_start(&self) -> u16 {
        u16::try_from(self.outer.start()).unwrap()
    }
    // returns the start and end of displayable text
    pub fn get_display_range(&self) -> (usize, usize) {
        (self.scroll, self.scroll + self.outer.len() - self.tip)
    }
}
#[derive(Clone, Debug)]
pub struct ColoredText {
    pub color: Color,
    pub text:  String,
}
impl ColoredText {
    pub fn white(text: &str) -> Self {
        Self {
            color: Color::Rgb {r: 205, g: 205, b: 205},
            text: String::from(text),
        }
    }
    pub fn new(text: &str, color: Color) -> Self {
        Self {
            color: color,
            text: String::from(text),
        }
    }
    pub fn getcolor(&self) -> Color {
        self.color
    }
}
#[derive(Clone, Debug)]
pub struct CursorText {
    cursor: Cursor,
    text:   String,
    range:  Range,
}
impl CursorText {
    pub fn new(rect: &Rect, source: &str) -> Self {
        let range = rect.horizontal().unwrap();
        Self {
            cursor: Cursor::text(source.len(), &range),
            text:   String::from(source),
            range:  range,
        }
    }
    pub fn resize(&mut self, rect: &Rect) {
        let range = rect.horizontal().unwrap();
        self.cursor.resize(self.text.len(), &range);
        self.range = range;
    }
    pub fn view(&self, y: u16, mut stdout: &Stdout) -> io::Result<()> {
        stdout.queue(cursor::Hide)?;
        let (a, b) = self.cursor.get_display_range();
        let text = &self.text[a..b]; 
        stdout
            .queue(cursor::MoveTo(
                    self.cursor.get_screen_start(), y))?
            .queue(style::Print(text))?
            .queue(cursor::MoveTo(self.cursor.get_cursor(), y))?
            .queue(cursor::Show)?
            .flush()
    }
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
    pub fn move_left(&mut self, step: usize) -> bool {
        self.cursor.backward(step)
    }
    pub fn move_right(&mut self, step: usize) -> bool {
        self.cursor.forward(step)
    }
    pub fn delete(&mut self) -> bool {
        if self.cursor.get_index() == self.text.len() {
            return false
        }
        self.text.remove(self.cursor.get_index());
        self.cursor.resize(self.text.len(), &self.range);
        if usize::from(self.cursor.get_cursor()) + 1 != self.range.end() {
            self.cursor.forward(1);
        }
        true
    }
    pub fn backspace(&mut self) -> bool {
        if self.cursor.is_start() {
            return false
        } 
        self.cursor.backward(1);
        self.text.remove(self.cursor.get_index());
        self.cursor.resize(self.text.len(), &self.range);
        if usize::from(self.cursor.get_cursor()) + 1 != self.range.end() {
            self.cursor.forward(1);
        }
        true
    }
    pub fn insert(&mut self, c: char) -> bool {
        if self.cursor.get_index() + 1 == self.text.len() {
            self.text.push(c);
        } else {
            self.text.insert(self.cursor.get_index(), c);
        }
        self.cursor.resize(self.text.len(), &self.range);
        self.cursor.forward(1);
        true
    }
}
#[derive(Clone, Debug)]
pub struct Pager {
    rect:    Rect,
    cursor:  Cursor,
    source:  Vec<ColoredText>,
    display: Vec<(usize, String)>,
} 
impl Pager {
    pub fn white(rect: &Rect, source: &Vec<String>) -> Self {
        let white: Vec<ColoredText> = source
            .iter()
            .map(|s| ColoredText::white(s))
            .collect();
        Self::new(rect, &white, 0)
    }
    pub fn new(rect: &Rect, source: &Vec<ColoredText>, buf: u8) -> Self {
        let display = common::wrap_list(
            &source.iter().map(|ct| ct.text.clone()).collect(),
            rect.w);
        return Self {
            rect:    rect.clone(),
            cursor:  
                Cursor::new(
                    display.len(), 
                    &rect.verticle().unwrap(), 
                    buf),
            source:  source.clone(),
            display: display,
        }
    }
    pub fn resize(&mut self, rect: &Rect) {
        self.rect    = rect.clone();
        self.display = common::wrap_list(
            &self.source.iter().map(|ct| ct.text.clone()).collect(),
            rect.w);
        self.cursor.resize(
            self.display.len(), 
            &rect.verticle().unwrap());
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout.queue(cursor::Hide)?;
        let (a, b) = self.cursor.get_display_range();
        for (j, (i, text)) in self.display[a..b].iter().enumerate() {
            stdout
                .queue(cursor::MoveTo(
                        self.rect.x, 
                        self.cursor.get_screen_start() + j as u16))?
                .queue(style::SetForegroundColor(
                        self.source[*i].color))?
                .queue(style::Print(text.as_str()))?;
        }
        stdout
            .queue(cursor::MoveTo(
                self.rect.x, 
                self.cursor.get_cursor()))?
            .queue(cursor::Show)?
            .flush()
    }
    pub fn move_up(&mut self, step: usize) -> bool {
        self.cursor.backward(step)
    }
    pub fn move_down(&mut self, step: usize) -> bool {
        self.cursor.forward(step)
    }
    pub fn select_under_cursor(&self) -> (usize, &str) {
        let index = self.display[self.cursor.get_index()].0;
        (index, &self.source[index].text)
    }
} 
