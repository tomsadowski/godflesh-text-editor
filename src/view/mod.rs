// mod

pub mod model;
pub mod dialog;
pub mod styles;
pub mod text;

use crate::constants;
use ratatui::{
    prelude::Size,
};
use model::{
    Model,
    Message,
};
use dialog::Dialog;
use crossterm::{
    event::{
        KeyModifiers, 
        KeyEvent, 
        Event, 
        KeyEventKind, 
        KeyCode},
};

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
            else {
                let text = format!("you pressed {}", c);
                m.dialog = Some(Dialog::Acknowledge(text)); 
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

