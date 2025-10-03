#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod content;
mod table;
mod bar;
mod model;
use model::*;
use content::*;
use table::*;
use bar::*;

use std::io::{
    self, Write, stdout, 
};
use crossterm::{
    execute, cursor, 
    event::{self, Event, KeyModifiers, KeyEventKind, KeyCode, KeyEvent},
    terminal::{self, ScrollUp, SetSize, size}
};
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend}, buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    crossterm::{
        ExecutableCommand,
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
    },
};

pub fn update(model: Model, msg: Message) -> Model {
    let mut m = model.clone();
    match msg {
        Message::GoToUrl => { m.go_to_url(); m }
        Message::Stop => { m.state = State::Stopped; m }
        Message::Switch(view) => { m.focus = view; m },
        Message::Move(direction, steps) => {
            match direction {
                Direction::Horizontal => {
                    match model.focus {
                        View::Content => 
                            { m.content.move_horizontal(steps); m }
                        View::AddressBar => 
                            { m.address_bar.move_horizontal(steps); m }
                        _ => 
                            model,
                    }
                },
                Direction::Vertical => {
                    match model.focus {
                        View::Content => 
                            { m.content.move_vertical(steps); m }
                        View::History => 
                            { m.history.move_vertical(steps); m }
                        View::Bookmarks => 
                            { m.bookmarks.move_vertical(steps); m }
                        _ => 
                            model,
                    }
                },
            }
        },
        Message::Edit(keycode) => {
            match model.focus {
                View::AddressBar => 
                    { m.address_bar.edit(keycode); m },
                _ => 
                    model,
            }
        },
    }
}

fn handle_key_event(model: &Model, keyevent: event::KeyEvent) -> Option<Message> {
    match keyevent {
        KeyEvent {
            code: KeyCode::Char('c'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            Some(Message::Stop)
        }
        _ => {handle_unmod_key(model, keyevent)}
    }
}

fn handle_unmod_key(model: &Model, key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Esc => 
            Some(Message::Switch(View::Content)),
        KeyCode::Enter => 
            Some(Message::GoToUrl),
        KeyCode::Char('i') if model.focus != View::AddressBar => 
            Some(Message::Move(Direction::Vertical, -1)),
        KeyCode::Char('o') if model.focus != View::AddressBar => 
            Some(Message::Move(Direction::Vertical, 1)),
        KeyCode::Char('e') if model.focus != View::AddressBar => 
            Some(Message::Move(Direction::Horizontal, -1)),
        KeyCode::Char('n') if model.focus != View::AddressBar => 
            Some(Message::Move(Direction::Horizontal, 1)),
        _ => 
            None,
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut model    = Model::default();

    while model.state != State::Stopped {

        terminal.draw(|f| f.render_widget(&model, f.area()))?;

        if let Some(message) = match event::read()? {
            Event::Key(keyevent) => handle_key_event(&model, keyevent),
            _                    => None,
        } {
            model = update(model, message);
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
