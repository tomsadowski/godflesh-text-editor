// main

#![allow(dead_code)]
#![allow(unused_imports)]

mod util;
mod gemtext;
mod status;
mod textview;
mod model;

use crate::{
    model::{
        Message, Model,
    },
};
use crossterm::{
    QueueableCommand, terminal, cursor, event
};
use std::io::{
    self, stdout, Write
};
use url::Url;

// elm paradigm
fn main() -> io::Result<()> {
    // init
    terminal::enable_raw_mode()?;
    let     url    = Url::parse("gemini://geminiprotocol.net/").ok();
    let     size   = terminal::size()?;
    let mut model  = Model::new(&url, size.0, size.1);
    let mut stdout = stdout();

    stdout
        .queue(terminal::EnterAlternateScreen)?
        .queue(terminal::DisableLineWrap)?
        .queue(cursor::Show)?;
    stdout.flush()?;

    while !model.quit() {
        // display model
        model.view(&stdout)?;

        // update model with event message.
        // note that calling `event::read()` blocks until
        // an event is encountered.
        if let Some(msg) = Message::from_event(event::read()?) {
            model.update(msg);
        }
    }

    // clean up
    terminal::disable_raw_mode()?;
    stdout.queue(terminal::LeaveAlternateScreen)?;
    stdout.flush()?;
    Ok(())
}
