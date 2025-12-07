// status

use url::{Url, ParseError};
use regex::Regex;

const STATUS_REGEX: &str = r"^(\d{1,3})[ \t](.*)\r\n$";

#[derive(Debug, Clone)]
pub enum Input {
    Input(i16),
    Sensitive,
}
#[derive(Debug, Clone)]
pub enum Success {
    Success(i16),
}
#[derive(Debug, Clone)]
pub enum Redirect {
    Temporary(i16),
    Permanent,
}
#[derive(Debug, Clone)]
pub enum TemporaryFailure {
    TemporaryFailure(i16),
    ServerUnavailable,
    CGIError,
    ProxyError,
    SlowDown,
}
#[derive(Debug, Clone)]
pub enum PermanentFailure {
    PermanentFailure(i16),
    NotFound,             
    Gone,                 
    ProxyRequestRefused,  
    BadRequest,           
}
#[derive(Debug, Clone)]
pub enum ClientCertRequired {
    ClientCertRequired(i16),
    TransientCertRequired,   
    AuthorizedCertRequired,  
    CertNotAccepted,         
    FutureCertRejected,      
    ExpiredCertRejected,     
} 
#[derive(Debug, Clone)]
pub enum Status {
    InputExpected(Input, String),
    Success(Success, String),
    Redirect(Redirect, Url),
    TemporaryFailure(TemporaryFailure, String),
    PermanentFailure(PermanentFailure, String),
    ClientCertRequired(ClientCertRequired, String),
}
impl Status {
    pub fn new(code: i16, meta: String) -> Result<Self, String> {
        match code {
            10 => 
                Ok(Self::InputExpected(Input::Input(code), meta)),
            11 => 
                Ok(Self::InputExpected(Input::Sensitive, meta)),
            12..=19 => 
                Ok(Self::InputExpected(Input::Input(code), meta)),
            20..=29 => 
                Ok(Self::Success(Success::Success(code), meta)),
            30..=39 => {
                let url = Url::parse(&meta) 
                    .or_else(|e| Err(format!("{}", e)))?;

                if code == 31 {
                    Ok(Self::Redirect(
                            Redirect::Permanent, url))
                } else {
                    Ok(Self::Redirect(
                            Redirect::Temporary(code), url))
                }
            }
            40 => 
                Ok(Self::TemporaryFailure(
                        TemporaryFailure::TemporaryFailure(code), meta)),
            41 => 
                Ok(Self::TemporaryFailure(
                        TemporaryFailure::ServerUnavailable, meta)),
            42 => 
                Ok(Self::TemporaryFailure(
                        TemporaryFailure::CGIError, meta)),
            43 => 
                Ok(Self::TemporaryFailure(
                        TemporaryFailure::ProxyError, meta)),
            44 => 
                Ok(Self::TemporaryFailure(
                        TemporaryFailure::SlowDown, meta)),
            45..=49 => 
                Ok(Self::TemporaryFailure(
                        TemporaryFailure::TemporaryFailure(code), meta)),
            50 => 
                Ok(Self::PermanentFailure(
                        PermanentFailure::PermanentFailure(code), meta)),
            51 => 
                Ok(Self::PermanentFailure(
                        PermanentFailure::NotFound, meta)),
            52 => 
                Ok(Self::PermanentFailure(
                        PermanentFailure::Gone, meta)),
            53 => 
                Ok(Self::PermanentFailure(
                        PermanentFailure::ProxyRequestRefused, meta)),
            54..=58 => 
                Ok(Self::PermanentFailure(
                        PermanentFailure::PermanentFailure(code), meta)),
            59 => 
                Ok(Self::PermanentFailure(
                        PermanentFailure::BadRequest, meta)),
            60 => 
                Ok(Self::ClientCertRequired(
                        ClientCertRequired::ClientCertRequired(code), meta)),
            61 => 
                Ok(Self::ClientCertRequired(
                        ClientCertRequired::TransientCertRequired, meta)),
            62 => 
                Ok(Self::ClientCertRequired(
                        ClientCertRequired::AuthorizedCertRequired, meta)),
            63 => 
                Ok(Self::ClientCertRequired(
                        ClientCertRequired::CertNotAccepted, meta)),
            64 => 
                Ok(Self::ClientCertRequired(
                        ClientCertRequired::FutureCertRejected, meta)),
            65 => 
                Ok(Self::ClientCertRequired(
                        ClientCertRequired::ExpiredCertRejected, meta)),
            66..=69 => 
                Ok(Self::ClientCertRequired(
                        ClientCertRequired::ClientCertRequired(code), meta)),
            _ => 
                Err(format!(
                    "received status number {} which maps to nothing", code)),
        }
    }

    pub fn from_str(line: &str) -> Result<Status, String> {
        // get regex
        let Ok(regex) = Regex::new(STATUS_REGEX)
            else {return Err("".to_string())};

        // get captures
        let Some(captures) = regex.captures(&line) 
            else {return Err("".to_string())};

        // get code from captures
        let Ok(code) = captures
            .get(1)
            .map_or("", |m| m.as_str())
            .parse()
            else {return Err("".to_string())};

        // get meta from captures
        let meta = captures
            .get(2)
            .map_or("", |m| m.as_str())
            .to_string();

        // return Result
        Status::new(code, meta)
    }
}
