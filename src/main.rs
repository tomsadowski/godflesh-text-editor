// main

#![allow(unused_variables)]
#![allow(dead_code)]

mod model;
mod display;
mod gemtext;
mod gemstatus;
mod constants;
mod util;



// *** BEGIN IMPORTS ***
use url::Url;
use std::io::{
    self, 
    stdout
};
use crossterm::event;
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
use crate::display::{
    LineStyles,
    DisplayModel,
};
// *** END IMPORTS ***



fn main() -> io::Result<()> 
{
    // enter alternate screen
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    // data init
    let url = Url::parse(constants::INIT_LINK).ok();

    let model = model::Model::init(&url);

    let mut display  = DisplayModel::new(&model, LineStyles::new());

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // main loop
    while !display.source.quit 
    {
        // display model
        terminal.draw(|f| f.render_widget(&display, f.area()))?;

        // update model with event message
        if let Some(message) = model::handle_event(event::read()?) 
        {
            display.source = model::update(display.source, message);
        }
    }

    // ui close
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

