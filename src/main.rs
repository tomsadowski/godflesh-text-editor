// main

#![allow(dead_code)]

mod util;
mod gemtext;
mod ui;
mod widget;
mod tabs;

use crate::ui::UI;
use crossterm::{QueueableCommand, terminal, cursor, event};
use std::io::{self, stdout, Write};

fn main() -> io::Result<()> {
    let (w, h) = terminal::size()?;
    let mut ui = UI::new("gemini://geminiprotocol.net/", w, h);
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    stdout
        .queue(terminal::EnterAlternateScreen)?
        .queue(terminal::DisableLineWrap)?
        .queue(cursor::Show)?;
    stdout.flush()?;

    ui.view(&stdout)?;

    // main loop
    while !ui.quit() {
        if ui.update(event::read()?) {
            ui.view(&stdout)?;
        }
    }

    // clean up
    terminal::disable_raw_mode()?;
    stdout.queue(terminal::LeaveAlternateScreen)?;
    stdout.flush()?;
    Ok(())
}
