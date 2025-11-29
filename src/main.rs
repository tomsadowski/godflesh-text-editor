// main

#![allow(dead_code)]

mod view;
mod gemini;
mod constants;
mod util;
mod msg;

use url::Url;
use crossterm::event;
use std::io::{
    self, 
    stdout
};
use crate::{
    msg::Message,
    view::update::update,
    view::model::Model,
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend, 
    crossterm::{
        ExecutableCommand,
        terminal::{
            disable_raw_mode, 
            enable_raw_mode, 
            EnterAlternateScreen, 
            LeaveAlternateScreen,
        },
    },
};



fn main() -> io::Result<()> 
{
    // enter alternate screen
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    // data init
    let     url      = Url::parse(constants::INIT_LINK).ok();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut model    = Model::init(&url, terminal.size()?);

    // main loop
    while !model.quit 
    {
        // display model
        terminal.draw(|f| f.render_widget(&model, f.area()))?;
        terminal.show_cursor()?;
        terminal.set_cursor_position(
            (model.text.cursor.x, model.text.cursor.y))?;

        // update model with event message
        if let Some(message) = Message::from_event(event::read()?) {
            model = update(model, message);
        }
    }

    // ui close
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}


