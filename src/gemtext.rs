// gemtext

use crate::util::GetColors;
use crossterm::style::{Color, Colors};
use url::{Url, ParseError};
use regex::Regex;

const GOPHER_SCHEME: &str    = "gopher";
const HTTPS_SCHEME: &str     = "https";
const HTTP_SCHEME: &str      = "http";
const LINK_SYMBOL: &str      = "=>";
const TOGGLE_SYMBOL: &str    = "```";
const QUOTE_SYMBOL: &str     = ">";
const LIST_ITEM_SYMBOL: &str = "*";
const HEADING_1_SYMBOL: &str = "#";
const HEADING_2_SYMBOL: &str = "##";
const HEADING_3_SYMBOL: &str = "###";
const GEMINI_PORT: &str      = "1965";
const GEMINI_SCHEME: &str    = "gemini";
const LINK_REGEX: &str       = r"^\s*(\S*)\s*(.*)?$";
const STATUS_REGEX: &str = r"^(\d{1,3})[ \t](.*)\r\n$";

#[derive(Clone, PartialEq, Debug)]
pub enum Scheme {
    Gemini(Url),
    Gopher(Url),
    Http(Url),
    Relative(String),
    Unknown(Url),
}
#[derive(Clone, PartialEq, Debug)]
pub enum GemTextData {
    HeadingOne,
    HeadingTwo,
    HeadingThree,
    Text, 
    PreFormat,
    Link(Scheme),
    ListItem,
    Quote,
} 
impl GetColors for GemTextData {
    fn getcolors(&self) -> Colors {
        match self {
            Self::HeadingOne => Colors::new(
                Color::Rgb {r: 225, g: 105, b: 105},
                Color::Rgb {r: 0, g: 0, b: 0},
            ),
            Self::HeadingTwo => Colors::new(
                Color::Rgb {r: 225, g: 105, b: 105},
                Color::Rgb {r: 0, g: 0, b: 0},
            ),
            Self::HeadingThree => Colors::new(
                Color::Rgb {r: 225, g: 105, b: 105},
                Color::Rgb {r: 0, g: 0, b: 0},
            ),
            Self::Quote => Colors::new(
                Color::Rgb {r: 180, g: 180, b: 180},
                Color::Rgb {r: 0, g: 0, b: 0},
            ),
            Self::Text => Colors::new(
                Color::Rgb {r: 180, g: 180, b: 180},
                Color::Rgb {r: 0, g: 0, b: 0},
            ),
            Self::PreFormat => Colors::new(
                Color::Rgb {r: 180, g: 180, b: 180},
                Color::Rgb {r: 0, g: 0, b: 0},
            ),
            Self::ListItem => Colors::new(
                Color::Rgb {r: 105, g: 180, b: 225},
                Color::Rgb {r: 0, g: 0, b: 0},
            ),
            Self::Link(_) => Colors::new(
                Color::Rgb {r: 105, g: 180, b: 225},
                Color::Rgb {r: 0, g: 0, b: 0},
            ),
        } 
    }
}
impl Scheme {
    fn from_str(line: &str) -> Result<(Scheme, String), String> {
        // get regex
        let Ok(regex) = Regex::new(LINK_REGEX)
            else {return Err(format!("regex: no parse"))};
        // get captures
        let Some(captures) = regex.captures(&line) 
            else {return Err(format!("regex: no captures"))};
        // get string
        let url_str = captures
            .get(1)
            .map_or("", |m| m.as_str())
            .to_string();
        // get result
        let url_result = Url::parse(&url_str);
        // get label 
        let label_str = captures
            .get(2)
            .map_or("", |m| m.as_str());
        let label = 
            if label_str.is_empty() {
                url_str.clone()
            } else {
                label_str.to_string()
            };
        // return Result
        if let Ok(url) = url_result {
            let scheme = match url.scheme() {
                GEMINI_SCHEME => Scheme::Gemini(url),
                GOPHER_SCHEME => Scheme::Gopher(url),
                HTTP_SCHEME   => Scheme::Http(url),
                HTTPS_SCHEME  => Scheme::Http(url),
                _             => Scheme::Unknown(url),
            };
            Ok((scheme, label))
        } else if Err(ParseError::RelativeUrlWithoutBase) == url_result {
            Ok((Scheme::Relative(url_str), label))
        } else {
            Err(format!("no parse url")) 
        }
    }
}
pub fn parse_doc(lines: Vec<&str>) 
    -> Result<Vec<(GemTextData, String)>, String> 
{
    let mut vec = vec![];
    let mut lines_iter = lines.iter();
    // return empty output if empty input
    let Some(first_line) = lines_iter.next() 
        else {return Ok(vec)};
    let mut preformat_flag = is_toggle(first_line);
    if !preformat_flag {
        // return error if cannot parse formatted line
        let formatted = parse_formatted(first_line)
            .or_else(|e| Err(format!("{}", e)))?;
        vec.push(formatted);
    }
    // parse remaining lines
    for line in lines_iter {
        if is_toggle(line) {
            preformat_flag = !preformat_flag;
        } else if preformat_flag {
            vec.push((GemTextData::PreFormat, line.to_string()));
        } else {
            let formatted = parse_formatted(line)
                .or_else(|e| Err(format!("{}", e)))?;
            vec.push(formatted);
        }
    }
    Ok(vec)
}
fn is_toggle(line: &str) -> bool {
    if let Some((symbol, _text)) = line.split_at_checked(3) {
        if symbol == TOGGLE_SYMBOL {
            return true
        }
    }
    return false
}
fn parse_formatted(line: &str) -> Result<(GemTextData, String), String> {
    // look for 3 character symbols
    if let Some((symbol, text)) = line.split_at_checked(3) {
        if symbol == HEADING_3_SYMBOL {
            return Ok((GemTextData::HeadingThree, text.to_string()))
        }
    }
    // look for 2 character symbols
    if let Some((symbol, text)) = line.split_at_checked(2) {
        if symbol == LINK_SYMBOL {
            let (url, text) = Scheme::from_str(text)
                .or_else(
                    |e| Err(format!("could not parse link {:?}", e)))?;
            return Ok((GemTextData::Link(url), text))
        }
        if symbol == HEADING_2_SYMBOL {
            return Ok((GemTextData::HeadingTwo, text.to_string()))
        }
    }
    // look for 1 character symbols
    if let Some((symbol, text)) = line.split_at_checked(1) {
        if symbol == QUOTE_SYMBOL {
            return Ok((GemTextData::Quote, text.to_string()))
        }
        if symbol == LIST_ITEM_SYMBOL {
            return Ok((GemTextData::ListItem, text.to_string()))
        }
        if symbol == HEADING_1_SYMBOL {
            return Ok((GemTextData::HeadingOne, text.to_string()))
        }
    }
    return Ok((GemTextData::Text, line.to_string()))
}
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
