// dialog

use url::Url;


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
    // Dialog asking to download resource
    pub fn download(str: String) -> Self 
    {
        Self 
        { 
            action: Action::Download, 
            text:   format!("Download nontext type: {}?", str)
        }
    }

    // Dialog asking for acknowledgement 
    pub fn acknowledge(str: String) -> Self 
    {
        Self 
        { 
            action: Action::Acknowledge, 
            text:   format!("{}?", str)
        }
    }

    // Dialog asking to go to new url
    pub fn follow_link(url: Url) -> Self 
    {
        Self 
        { 
            action: Action::FollowLink(url.clone()), 
            text:   format!("Go to {}?", String::from(url))
        }
    }
}
