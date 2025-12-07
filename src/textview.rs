// textview

use std::io::{
    self, Write, Stdout
};
use crossterm::{
    QueueableCommand, cursor, terminal,
    style::{
        self, Color, Colors,
    },
};
use crate::util::get_indexed_wrapped;

#[derive(Clone, Debug)]
pub struct TextView<'a, 'b> {
    source_text:  Vec<(Colors, &'a str)>,
    display_text: Vec<(usize , &'b str)>,
    scroll:       u16,
    width:        u16,
    height:       u16,
    cursor_x:     u16, 
    cursor_y:     u16,
} 
impl<'a: 'b, 'b> TextView<'a, 'b> {

    pub fn new(source: Vec<(Colors, &'a str)>, width: u16, height: u16) -> Self {

        let wrapped = get_indexed_wrapped(
            &source.iter().map(|x| x.1).collect(), 
            usize::from(width));

        return Self {
            source_text:  source,
            display_text: wrapped,
            width:        width,
            height:       height,
            cursor_x:     0,
            cursor_y:     0,
            scroll:       0,
        }
    }

    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> 
    {
        let start = usize::from(self.scroll);
        let end   = usize::from(self.scroll + self.height);
        stdout.queue(terminal::Clear(terminal::ClearType::All))?;

        for (i, l) in self.display_text[start..end].iter().enumerate() {
            stdout
                .queue(cursor::MoveTo(0, i as u16))?
                .queue(style::SetColors(self.source_text[l.0].0))?
                .queue(style::Print(l.1))?;
        }

        stdout.queue(cursor::MoveTo(self.cursor_x, self.cursor_y))?;
        stdout.flush()?;
        Ok(())
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.display_text = 
            get_indexed_wrapped(
                &self.source_text.iter().map(|x| x.1).collect(), 
                usize::from(self.width));
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_y < self.height - 1 {
            self.cursor_y += 1;
        }
        else if 
            (self.scroll + self.height - 1) < 
            ((self.display_text.len() as u16) - 1) 
        {
            self.scroll += 1;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
        }
        else if self.scroll > 0 {
            self.scroll -= 1;
        }
    }
} 
