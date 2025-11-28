// model



// *** BEGIN IMPORTS ***
use url::Url;
use std::str::FromStr;
use crate::{
    util, 
    constants,
    view::{
        dialog::{
            Dialog,
            Action,
        },
        styles::{
            LineStyles,
        },
        text::{
            ModelText,
        },
    },
    gemini::{
        status::{
            Status,
        },
    },
};
use ratatui::{
    prelude::*, 
};
use crossterm::{
    event::{
        KeyModifiers, 
        KeyEvent, 
        Event, 
        KeyEventKind, 
        KeyCode},
};
// *** END IMPORTS ***


#[derive(Clone, Debug)]
pub enum Message {
    Code(char),
    Resize(u16, u16),
    Enter,
    Escape,
    Stop,
}


#[derive(Clone, Debug)]
pub enum Address {
    Url(Url), 
    String(String),
}


#[derive(Clone, Debug)]
pub struct Model<'a> {
    pub dialog:  Option<Dialog>,
    pub address: Address,
    pub text:    ModelText<'a>,
    pub quit:    bool,
} 
impl<'a> Model<'a>
{
    pub fn init(_url: &Option<Url>, size: Size) -> Self 
    {
        let styles = LineStyles::new();

        // return now if no url provided
        let Some(url) = _url else 
        {
            let text = 
                ModelText::plain_text(
                    format!("welcome"), 
                    size, 
                    &styles);

            return Self {
                address: Address::String(String::from("")),
                text:    text,
                dialog:  None,
                quit:    false,
            }
        };

        let address = Address::Url(url.clone());

        // return now if data retrieval fails
        let Ok((header, content)) = util::get_data(&url) else 
        {
            let text = 
                ModelText::plain_text(
                    format!("data retrieval failed"), 
                    size, 
                    &styles);

            return Self {
                address: address,
                text:    text,
                dialog:  None,
                quit:    false,
            }
        };

        // return now if status parsing fails
        let Ok(status) = Status::from_str(&header) else {
            let text = 
                ModelText::plain_text(
                    format!("could not parse status"), 
                    size, 
                    &styles);

            return Self {
                address: address,
                text:    text,
                dialog:  None,
                quit:    false,
            }
        };

        let text = 
            ModelText::init_from_response(
                status.clone(), 
                content, 
                size, 
                &styles);

        let dialog = Dialog::init_from_response(status);

        Self {
            address: address,
            text:    text,
            dialog:  dialog,
            quit:    false,
        }
    }
    
    pub fn get_cursor_position(self) -> Position 
    {
        self.text.cursor
    }

    pub fn submit(mut self)
    {
        if let Some(dialog) = self.dialog {
            match dialog.action
            {
                Action::Download => {
                    self.dialog = None;
                },
                Action::Acknowledge => {
                    self.dialog = None;
                },
                Action::FollowLink(url) => {
                    // return now if data retrieval fails
                    let Ok((header, content)) = util::get_data(&url) else 
                    {
                        return
                    };

                    // return now if status parsing fails
                    let Ok(status) = Status::from_str(&header) else {
                        return
                    };

                    self.text = self.text.update_from_response(status, content);
                    self.dialog = None;
                },
            }
        }
    }

} 
impl<'a> Widget for &Model<'a> 
{
    fn render(self, area: Rect, buf: &mut Buffer) 
    {
        self.text.render(area, buf);
    }
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
