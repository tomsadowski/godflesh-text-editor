// model

// *** BEGIN IMPORTS ***
use url::Url;
use std::str::FromStr;
use crate::{
    util, 
    gemtext::GemTextDoc,
    gemstatus::Status,
    constants,
};
use crossterm::{
    event::{self, 
        KeyModifiers, 
        KeyEvent, 
        Event, 
        KeyEventKind, 
        KeyCode},
};
// *** END IMPORTS ***


#[derive(Clone, PartialEq, Debug)]
pub enum Message {
    Code(char),
    Enter,
    Escape,
    Stop,
}
#[derive(Clone, PartialEq, Debug)]
pub enum Dialog {
    AddressBar(Vec<u8>), 
    Prompt(String, Vec<u8>),
    Message(String),
}
#[derive(Clone, Debug)]
pub enum Address {
    Url(Url), 
    String(String),
}
#[derive(Clone, Debug)]
pub enum Text {
    GemText(GemTextDoc), 
    String(String),
}
#[derive(Clone, Debug)]
pub struct Model {
    pub dialog:  Option<Dialog>,
    pub address: Address,
    pub text:    Text,
    pub quit:    bool,
    pub x:       usize,
    pub y:       usize,
} 
impl Dialog {
    pub fn init_from_response(status: Status) -> Option<Self> {
        match status {
            Status::InputExpected(variant, msg) => {
                Some(
                    Self::Prompt(
                        format!("input: {}", msg), 
                        vec![]
                    )
                )
            }
            Status::Success(variant, meta) => {
                if meta.starts_with("text/") {
                    None
                } else {
                    Some(
                        Self::Prompt(
                            format!("Download nontext type: {}?", meta), 
                            vec![]
                        )
                    )
                }
            }
            Status::TemporaryFailure(variant, meta) => {
                None
            }
            Status::PermanentFailure(variant, meta) => {
                None
            }
            Status::Redirect(variant, new_url) => {
                Some(
                    Self::Prompt(
                        format!("Redirect to: {}?", new_url), 
                        vec![]
                    )
                )
            }
            Status::ClientCertificateRequired(variant, meta) => {
                Some(
                    Self::Prompt(
                        format!("Certificate required: {}", meta),
                        vec![]
                    )
                )
            }
        }
    }
}
impl Text {
    pub fn init_from_response(status: Status, content: String) -> Self {
        match status {
            Status::InputExpected(variant, msg) => {
                Self::String(content)
            }
            Status::Success(variant, meta) => {
                if meta.starts_with("text/") {
                    Self::GemText(GemTextDoc::new(content))
                } else {
                    Self::String(String::from("no text"))
                }
            }
            Status::TemporaryFailure(variant, meta) => {
                Self::String(format!("Temporary Failure {:?}: {:?}", variant, meta))
            }
            Status::PermanentFailure(variant, meta) => {
                Self::String(format!("Permanent Failure {:?}: {:?}", variant, meta))
            }
            Status::Redirect(variant, new_url) => {
                Self::String(format!("Redirect to: {}?", new_url))
            }
            Status::ClientCertificateRequired(variant, meta) => {
                Self::String(format!("Certificate required: {}", meta))
            }
        }
    }
}
impl Model {
    pub fn init(_url: &Option<Url>) -> Self {
        let Some(url) = _url else {
            return Self {
                address: Address::String(String::from("")),
                text:    Text::String(String::from("welcome")),
                dialog:  None,
                quit:    false,
                x:       0,
                y:       0,
            }
        };
        let Ok((header, content)) = util::get_data(&url) else {
            return Self {
                address: Address::Url(url.clone()),
                text:    Text::String(String::from("no get data")),
                dialog:  None,
                quit:    false,
                x:       0,
                y:       0,
            }
        };
        let Ok(status) = Status::from_str(&header) else {
            return Self {
                address: Address::Url(url.clone()),
                text:    Text::String(String::from("could not parse status")),
                dialog:  None,
                quit:    false,
                x:       0,
                y:       0,
            }
        };
        Self {
            address: Address::Url(url.clone()),
            text:    Text::init_from_response(status.clone(), content),
            dialog:  Dialog::init_from_response(status),
            quit:    false,
            x:       0,
            y:       0,
        }
    }
} 
pub fn update(model: Model, msg: Message) -> Model {
    let mut m = model.clone();
    match msg {
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
                    constants::LEFT  => {},
                    constants::RIGHT => {}, 
                    constants::UP    => {},
                    constants::DOWN  => {},
                    _ => {}
                }
            } else {
                m.dialog = Some(Dialog::Message(format!("you pressed {}", c))); 
            }
        }
    }
    m
}
pub fn handle_event(event: event::Event) -> Option<Message> {
    let Event::Key(keyevent) = event 
        else {return None};
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
        _ => None
    }
}
