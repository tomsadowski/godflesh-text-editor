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


// return new model based on old model and message
pub fn update(model: Model, msg: Message) -> Model 
{
    let mut m = model.clone();

    match msg {
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
            else { 
                let data = m.text.get_text_under_cursor().0;
                m.dialog = query_gemtext_data(data);
            }
        }
        Message::Resize(_y, _x) => {
//            m.text.size = (y, x);
        }
        Message::Code(c) => {
            if let None = m.dialog {
                match c {
                //  constants::LEFT => {
                //      m.text.move_cursor_left();
                //  }
                    constants::UP => {
                        m.text.move_cursor_up();
                    }
                //  constants::RIGHT => {
                //      m.text.move_cursor_right();
                //  }
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

pub fn query_gemtext_data(text: GemTextData) -> Option<Dialog>
{
    match text {
        GemTextData::Link(Scheme::Gemini(url)) => 
        {
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

