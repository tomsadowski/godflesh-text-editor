// model

use crate::{
    util, 
    view::dialog::Dialog,
    view::styles::LineStyles,
    view::text::GemTextView,
    gemini::status::Status,
};
use url::Url;



#[derive(Clone, Debug)]
pub enum Address 
{
    Url(Url), 
    String(String),
}

#[derive(Clone, Debug)]
pub struct Model
{
    pub dialog:  Option<Dialog>,
    pub address: Address,
    pub text:    GemTextView,
    pub quit:    bool,
} 
impl Model
{
    pub fn init(_url: &Option<Url>, size: (u16, u16)) -> Self 
    {
        let styles = LineStyles::new();

        // return now if no url provided
        let Some(url) = _url else 
        {
            let text = 
                GemTextView::new(
                    format!("welcome"), 
                    &styles,
                    size,
                    );

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
                GemTextView::new(
                    format!("data retrieval failed"), 
                    &styles,
                    size, 
                    );

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
                GemTextView::new(
                    format!("could not parse status"), 
                    &styles,
                    size, 
                    );

            return Self {
                address: address,
                text:    text,
                dialog:  None,
                quit:    false,
            }
        };
        
        let (text, dialog) = match status 
        {
            Status::Success(_variant, meta) => 
            {
                if meta.starts_with("text/") 
                {
                    (GemTextView::new(content, &styles, size),
                    None)
                } 
                else 
                {
                    (GemTextView::new(content, &styles, size),
                    Some(Dialog::download(meta)))
                }
            }
            Status::InputExpected(_variant, msg) => 
            {
                let text = format!("input: {}", msg);
                (GemTextView::new(content, &styles, size),
                Some(Dialog::acknowledge(text)))
            }
            Status::Redirect(_variant, new_url) => 
            {
                (GemTextView::new(content, &styles, size),
                Some(Dialog::follow_link(new_url)))
            }
            Status::ClientCertRequired(_variant, meta) => 
            {
                let text = format!("Certificate required: {}", meta);
                (GemTextView::new(content, &styles, size),
                Some(Dialog::acknowledge(text)))
            }
            _ => {
                (GemTextView::new(content, &styles, size),
                None)
            }
        };

        Self {
            address: address,
            text:    text,
            dialog:  dialog,
            quit:    false,
        }
    }
} 
