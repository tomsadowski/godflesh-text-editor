// dialog



// *** BEGIN IMPORTS ***
use crate::{
    gemini::status::Status,
};
// *** END IMPORTS ***



#[derive(Clone, Debug)]
pub enum Dialog 
{
    AddressBar(Vec<u8>), 
    Prompt(String, Vec<u8>),
    Confirmation(String),
    Acknowledge(String),
}
impl Dialog 
{
    pub fn init_from_response(status: Status) -> Option<Self> 
    {
        match status 
        {
            Status::Success(_variant, meta) => {
                if meta.starts_with("text/") {
                    None
                } 
                else {
                    let text = format!("Download nontext type: {}?", meta);
                    Some(Self::Confirmation(text))
                }
            }
            Status::InputExpected(_variant, msg) => {
                let text = format!("input: {}", msg);
                Some(Self::Prompt(text, vec![]))
            }
            Status::TemporaryFailure(_variant, _meta) => {
                None
            }
            Status::PermanentFailure(_variant, _meta) => {
                None
            }
            Status::Redirect(_variant, new_url) => {
                let text = format!("Redirect to: {}?", new_url);
                Some(Self::Confirmation(text))
            }
            Status::ClientCertRequired(_variant, meta) => {
                let text = format!("Certificate required: {}", meta);
                Some(Self::Prompt(text, vec![]))
            }
        }
    }
}
