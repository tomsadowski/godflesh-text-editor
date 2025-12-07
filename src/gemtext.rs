// gemtext

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
            } 
            else {
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
        } 
        else if Err(ParseError::RelativeUrlWithoutBase) == url_result {
            Ok((Scheme::Relative(url_str), label))
        } 
        else {
            Err(format!("no parse url")) 
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct GemTextLine {
    pub data: GemTextData,
    pub text: String,
} 
impl GemTextLine {
    pub fn parse_doc(lines: Vec<&str>) -> Result<Vec<Self>, String> {
        let mut vec        = vec![];
        let mut lines_iter = lines.iter();

        // return empty output if empty input
        let Some(first_line) = lines_iter.next() 
            else {return Ok(vec)};

        let mut preformat_flag = Self::is_toggle(first_line);

        if !preformat_flag {
            // return error if cannot parse formatted line
            let formatted = Self::parse_formatted(first_line)
                .or_else(
                    |e| Err(format!("{}", e))
                )?;
            vec.push(formatted);
        }

        // parse remaining lines
        for line in lines_iter {
            if Self::is_toggle(line) {
                preformat_flag = !preformat_flag;
            } 
            else if preformat_flag {
                vec.push(
                    Self {
                        data: GemTextData::PreFormat, 
                        text: line.to_string()
                    });
            }
            else {
                let formatted = Self::parse_formatted(line)
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

    fn parse_formatted(line: &str) -> Result<GemTextLine, String> {
        // look for 3 character symbols
        if let Some((symbol, text)) = line.split_at_checked(3) {
            if symbol == HEADING_3_SYMBOL {
                return Ok(
                    Self {
                        data: GemTextData::HeadingThree,
                        text: text.to_string(),
                    })
            }
        }

        // look for 2 character symbols
        if let Some((symbol, text)) = line.split_at_checked(2) {
            if symbol == LINK_SYMBOL {
                let (url, text) = Scheme::from_str(text)
                    .or_else(
                        |e| Err(format!("could not parse link {:?}", e))
                    )?;
                return Ok(
                    Self {
                        data: GemTextData::Link(url),
                        text: text,
                    })
            }
            if symbol == HEADING_2_SYMBOL {
                return Ok(
                    Self {
                        data: GemTextData::HeadingTwo,
                        text: text.to_string(),
                    })
            }
        }

        // look for 1 character symbols
        if let Some((symbol, text)) = line.split_at_checked(1) {
            if symbol == QUOTE_SYMBOL {
                return Ok(
                    Self {
                        data: GemTextData::Quote,
                        text: text.to_string(),
                    })
            }
            if symbol == LIST_ITEM_SYMBOL {
                return Ok(
                    Self {
                        data: GemTextData::ListItem,
                        text: text.to_string(),
                    })
            }
            if symbol == HEADING_1_SYMBOL {
                return Ok(
                    Self {
                        data: GemTextData::HeadingOne,
                        text: text.to_string(),
                    })
            }
        }
        return Ok(
            Self {
                data: GemTextData::Text,
                text: line.to_string(),
            })
    }
}
