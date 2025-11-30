// dialog

use url::Url;
use crate::{
    gemini::status::Status,
};



#[derive(Clone, Debug)]
pub enum Action
{
    FollowLink(Url),
    Download,
    Acknowledge,
}

#[derive(Clone, Debug)]
pub struct Dialog 
{
    pub action: Action,
    pub text:   String,
}
impl Dialog
{
    pub fn download(str: String) -> Self 
    {
        Self 
        { 
            action: Action::Download, 
            text:   format!("Download nontext type: {}?", str)
        }
    }

    pub fn acknowledge(str: String) -> Self 
    {
        Self 
        { 
            action: Action::Acknowledge, 
            text:   format!("{}?", str)
        }
    }

    pub fn follow_link(url: Url) -> Self 
    {
        Self 
        { 
            action: Action::FollowLink(url.clone()), 
            text:   format!("Go to {}?", String::from(url))
        }
    }

    pub fn init_from_response(status: Status) -> Option<Self> 
    {
        match status 
        {
            Status::Success(_variant, meta) => 
            {
                if meta.starts_with("text/") 
                {
                    None
                } 
                else 
                {
                    Some(Self::download(meta))
                }
            }
            Status::InputExpected(_variant, msg) => 
            {
                let text = format!("input: {}", msg);
                Some(Self::acknowledge(text))
            }
            Status::Redirect(_variant, new_url) => 
            {
                Some(Self::follow_link(new_url))
            }
            Status::ClientCertRequired(_variant, meta) => 
            {
                let text = format!("Certificate required: {}", meta);
                Some(Self::acknowledge(text))
            }
            _ => None,
        }
    }
}
