// msg



// *** BEGIN IMPORTS ***
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
