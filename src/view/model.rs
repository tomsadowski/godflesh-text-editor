// model

use crate::{
    util, 
    view::dialog::Dialog,
    view::styles::LineStyles,
    view::text::ModelText,
    gemini::status::Status,
};
use url::Url;
use ratatui::prelude::*;



#[derive(Clone, Debug)]
pub enum Address 
{
    Url(Url), 
    String(String),
}

#[derive(Clone, Debug)]
pub struct Model<'a> 
{
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
} 
