// main

#![allow(dead_code)]

mod view;
mod gemini;
mod constants;
mod util;

// *** BEGIN IMPORTS ***
use url::Url;
use std::io::{
    self, 
    stdout
};
use crate::view::{
    model::{
        self,
        Model,
    } 
};
use crossterm::{
    event::{
        self
    },
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
// *** END IMPORTS ***



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
        if let Some(message) = model::handle_event(event::read()?) {
            model = model::update(model, message);
        }
    }

    // ui close
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}


