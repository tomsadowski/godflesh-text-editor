// update

use crate::{
    util, 
    constants,
    msg::Message,
    view::model::Model,
    view::dialog::Dialog,
    view::dialog::Action,
    gemini::status::Status,
    gemini::gemtext::GemTextLine,
    gemini::gemtext::GemTextData,
    gemini::gemtext::Scheme,
};
use ratatui::prelude::Size;


// return new model based on old model and message
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
        Message::Escape => { 
            m.dialog = None;
        }
        Message::Enter => {
            if let Some(dialog) = m.dialog.clone() {
                m = respond_to_dialog(m, dialog);
            }
            else if let Ok(text) = m.text.get_gemtext_under_cursor() 
            {
                m.dialog = query_gemtext_line(text);
            }
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

pub fn query_gemtext_line(text: GemTextLine) -> Option<Dialog>
{
    match text {
        GemTextLine {
            data: GemTextData::Link(Scheme::Gemini(url)),
            ..
        } => {
            Some(Dialog::follow_link(url))
        }
        g => {
            Some(Dialog::acknowledge(format!("{:?}", g)))
        }
    }
}

pub fn respond_to_dialog(model: Model, dialog: Dialog) -> Model
{
    let mut m = model.clone();
    match dialog.action 
    {
        Action::FollowLink(url) => 
        {
            if let Ok((header, content)) = util::get_data(&url) 
            {
                if let Ok(status) = Status::from_str(&header) 
                {
                    m.text = m.text.update_from_response(status, content);
                }
            }
        },
        _ => {}
    }
    m.dialog = None;
    m
}

