// model



// *** BEGIN IMPORTS ***
use url::Url;
use std::str::FromStr;
use crate::{
    util, 
    view::{
        dialog::{
            Dialog,
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
      //gemtext::{
      //    GemTextLine,
      //},
    },
};
use ratatui::{
    prelude::*, 
};
// *** END IMPORTS ***



#[derive(Clone, Debug)]
pub enum Message 
{
    Code(char),
    Resize(u16, u16),
    Enter,
    Escape,
    Stop,
}



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
                    format!("\twelcome\n\twelcome\n\twelcome"), 
                    size, 
                    &styles);

            return Self 
            {
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
                    format!("\n\tdata\n\tretrieval\n\tfailed"), 
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
impl<'a> Widget for &Model<'a> 
{
    fn render(self, area: Rect, buf: &mut Buffer) 
    {
        self.text.render(area, buf);
    }
}

