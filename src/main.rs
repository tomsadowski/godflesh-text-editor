// main

#![allow(unused_variables)]
#![allow(dead_code)]

mod model;
mod gemtext;
mod gemstatus;
mod constants;
mod util;



// *** BEGIN IMPORTS ***
use url::Url;
use crossterm::{
    event::{
        self, 
        KeyModifiers, 
        KeyEvent, 
        Event, 
        KeyEventKind, 
        KeyCode},
};
use std::io::{
    self, 
    stdout
};
use crate::model::{
    Model, 
    Message,
    Dialog,
};
use ratatui::{
    prelude::Size,
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
    let url = Url::parse(constants::INIT_LINK).ok();

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut model = model::Model::init(&url, terminal.size()?);

    // main loop
    while !model.quit 
    {
        // display model
        terminal.draw(|f| f.render_widget(&model, f.area()))?;

        terminal.show_cursor()?;

        terminal.set_cursor_position((model.text.cursor.x, model.text.cursor.y))?;

        // update model with event message
        if let Some(message) = handle_event(event::read()?) 
        {
            model = update(model, message);
        }
    }

    // ui close
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}


pub fn update(model: Model, msg: Message) -> Model 
{
    let mut m = model.clone();

    match msg {
        Message::Resize(y, x) => {
            m.text.size = Size::new(y, x);
        }
        Message::Stop => { 
            m.quit = true;
        }
        Message::Enter => {
            m.dialog = None;
        }
        Message::Escape => { 
            m.dialog = None;
        }
        Message::Code(c) => {
            if let None = m.dialog {
                match c {
                    constants::LEFT => {
                        m.text.move_cursor_left();
                    }
                    constants::UP => {
                        m.text.move_cursor_up();
                    }
                    constants::RIGHT => {
                        m.text.move_cursor_right();
                    }
                    constants::DOWN => {
                        m.text.move_cursor_down();
                    }
                    _ => {}
                }
            } 
            else 
            {
                m.dialog = Some(Dialog::Message(format!("you pressed {}", c))); 
            }
        }
    }
    // return Model
    m
}

pub fn handle_event(event: Event) -> Option<Message> 
{
    match event {
        Event::Key(keyevent) => 
            handle_key_event(keyevent),

        Event::Resize(y, x) => 
            Some(Message::Resize(y, x)),

        _ => 
            None
    }
}

pub fn handle_key_event(keyevent: KeyEvent) -> Option<Message> 
{
    match keyevent {
        KeyEvent {
            code: KeyCode::Char('c'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            Some(Message::Stop)
        }
        KeyEvent {
            code: KeyCode::Enter,
            kind: KeyEventKind::Press,
            ..
        } => {
            Some(Message::Enter)
        }
        KeyEvent {
            code: KeyCode::Esc,
            kind: KeyEventKind::Press,
            ..
        } => {
            Some(Message::Escape)
        }
        KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            ..
        } => {
            Some(Message::Code(c))
        }
        _ => 
            None
    }
}

