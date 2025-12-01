// main

#![allow(dead_code)]
#![allow(unused_imports)]

mod view;
mod gemini;
mod constants;
mod util;
mod msg;

use url::Url;
use crossterm::{
    execute, 
    event,
    style::Print,
    terminal::{ScrollUp, SetSize, size},
};
use std::io::{
    self, 
    stdout,
};
use crate::{
    msg::Message,
    view::update::update,
    view::model::Model,
};



fn main() -> io::Result<()> 
{
    let (cols, rows) = size()?;

    let url       = Url::parse(constants::INIT_LINK).ok();
    let mut model = Model::init(&url, (cols, rows));

    while !model.quit {
        // Resize terminal and scroll up.
        execute!(
            io::stdout(),
            Print(format!("{:#?}", model)),
        )?;

        // update model with event message
        if let Some(message) = Message::from_event(event::read()?) {
            model = update(model, message);
        }
    }

    // Be a good citizen, cleanup
    execute!(io::stdout(), SetSize(cols, rows))?;
    Ok(())
}

