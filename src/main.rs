// gem/src/main

#![allow(dead_code)]
//#![allow(unused_imports)]

mod ui;
mod screen;
mod text;

use crossterm::{
    QueueableCommand, terminal, event,
};
use std::{
    io::{self, stdout, Write},
};

fn main() -> io::Result<()> {
    let arg = std::env::args().nth(1).expect("poop");
    let (w, h) = terminal::size()?;
    let mut ui = ui::UI::new(&arg, w, h);
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    stdout
        .queue(terminal::EnterAlternateScreen)?
        .queue(terminal::DisableLineWrap)?;
    ui.view(&stdout)?;

    // main loop
    while !ui.is_quit() {
        if ui.update(event::read()?) {
            ui.view(&stdout)?;
        }
    }

    // clean up
    terminal::disable_raw_mode()?;
    stdout.queue(terminal::LeaveAlternateScreen)?;
    stdout.flush()
}
