// model

use crate::{
    util, 
    textview::TextView,
    status::Status,
    gemtext::GemTextLine,
    gemtext::GemTextData,
    gemtext::Scheme,
};
use std::io::{
    self, Write, Stdout
};
use crossterm::{
    event::{
        Event, KeyEvent, KeyEventKind, KeyCode
    },
    style::{
        Colors, Color
    },
};
use url::Url;

const LEFT:  char = 'e';
const DOWN:  char = 'i';
const UP:    char = 'o';
const RIGHT: char = 'n';
const QUIT:  char = 'q';

const URL:   char = 'g';

#[derive(Clone, Debug)]
pub struct LineStyles {
    pub heading_one:   Colors,
    pub heading_two:   Colors,
    pub heading_three: Colors,
    pub link:          Colors,
    pub list_item:     Colors,
    pub quote:         Colors,
    pub preformat:     Colors,
    pub text:          Colors,
    pub plaintext:     Colors,
}
impl LineStyles {
    pub fn new() -> Self {
        let heading_one_style = 
            Colors::new(
                Color::Rgb {r: 208,  g:  96,  b:  96},
                Color::Rgb {r:  48,  g:  24,  b:  24},
            );
        let heading_two_style = 
            Colors::new(
                Color::Rgb {r: 208,  g:  96,  b:  96},
                Color::Rgb {r:   0,  g:   0,  b:   0},
            );
        let heading_three_style = 
            Colors::new(
                Color::Rgb {r: 208,  g:  96,  b:  96},
                Color::Rgb {r:   0,  g:   0,  b:   0},
            );
        let link_style = 
            Colors::new(
                Color::Rgb {r: 176,  g:  96,  b: 192},
                Color::Rgb {r:   0,  g:   0,  b:   0},
            );
        let text_style =
            Colors::new(
                Color::Rgb {r: 192,  g: 192,  b: 144},
                Color::Rgb {r:   0,  g:   0,  b:   0},
            );
        let list_style =
            Colors::new(
                Color::Rgb {r: 192,  g: 192,  b: 144},
                Color::Rgb {r:   0,  g:   0,  b:   0},
            );
        let quote_style =
            Colors::new(
                Color::Rgb {r: 192,  g: 192,  b: 144},
                Color::Rgb {r:   0,  g:   0,  b:   0},
            );
        Self {
            heading_one:   heading_one_style,
            heading_two:   heading_two_style,
            heading_three: heading_three_style,
            link:          link_style,
            list_item:     list_style,
            quote:         quote_style,
            preformat:     text_style,
            plaintext:     text_style,
            text:          text_style,
        }
    }
    pub fn get_colors(&self, data: &GemTextData) -> Colors {
        match data {
            GemTextData::HeadingOne   => self.heading_one,
            GemTextData::HeadingTwo   => self.heading_two,
            GemTextData::HeadingThree => self.heading_three,
            GemTextData::Text         => self.text,
            GemTextData::Quote        => self.quote,
            GemTextData::ListItem     => self.list_item,
            GemTextData::PreFormat    => self.preformat,
            _ => self.link,
        }
    }
    pub fn get_tuples<'a>(&self, lines: &'a Vec<GemTextLine>) 
        -> Vec<(Colors, &'a str)>
    {
        lines
            .iter()
            .map(|g| (self.get_colors(&g.data), g.text.as_str()))
            .collect()
    }
}

#[derive(Clone, Debug)]
pub enum Action {
    FollowLink(Url),
    Download,
    Acknowledge,
}

#[derive(Clone, Debug)]
pub struct Dialog {
    pub action: Action,
    pub text:   String,
}

impl Dialog {
    // Dialog asking to download resource
    pub fn download(str: String) -> Self {
        Self { 
            action: Action::Download, 
            text:   format!("Download nontext type: {}?", str)
        }
    }

    // Dialog asking for acknowledgement 
    pub fn acknowledge(str: String) -> Self {
        Self { 
            action: Action::Acknowledge, 
            text:   format!("{}?", str)
        }
    }

    // Dialog asking to go to new url
    pub fn follow_link(url: Url) -> Self {
        Self { 
            action: Action::FollowLink(url.clone()), 
            text:   format!("Go to {}?", String::from(url))
        }
    }

    pub fn query_gemtext_data(text: GemTextData) -> Option<Dialog> {
        match text {
            GemTextData::Link(Scheme::Gemini(url)) => {
                Some(Dialog::follow_link(url))
            }
            g => {
                Some(Dialog::acknowledge(format!("{:?}", g)))
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Message {
    Code(char),
    Resize(u16, u16),
}

impl Message {
    // given a relevant Event, return some Message
    pub fn from_event(event: Event) -> Option<Message> {
        match event {
            Event::Key(keyevent) => Self::from_key_event(keyevent),
            Event::Resize(y, x)  => Some(Message::Resize(y, x)),
            _                    => None,
        }
    }

    // given a relevant KeyEvent, return some Message
    fn from_key_event(keyevent: KeyEvent) -> Option<Message> {
        match keyevent {
            KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                ..
            } => {
                Some(Message::Code(c))
            }
            _ => 
                None
        }
    }
}

#[derive(Clone, Debug)]
pub enum Address 
{
    Url(url::Url), 
    String(String),
}

#[derive(Clone, Debug)]
pub struct Model {
    quit:    bool,
    source:  Vec<GemTextData>,
    text:    TextView,
    styles:  LineStyles,
}
impl Model {

    pub fn new(url: Url, width: u16, height: u16) -> Result<Self, String> {

        let (header, content) = util::get_data(&url).unwrap(); 
        let status            = Status::from_str(&header).unwrap();

        let lines: Vec<GemTextLine> = match status {
            Status::Success(_variant, meta) => 
                if meta.starts_with("text/") {
                    GemTextLine::parse_doc(content.lines().collect()).unwrap()
                } else {
                    GemTextLine::parse_doc(meta.lines().collect()).unwrap()
                },
            _ => 
                GemTextLine::parse_doc(content.lines().collect()).unwrap(),
        };

        let styles = LineStyles::new();
        let text   = TextView::new(
            lines
                .iter()
                .map(|g| (styles.get_colors(&g.data), g.text.clone()))
                .collect(),
            width, 
            height); 

        Ok(Self {
            source:  lines.iter().map(|g| g.data.clone()).collect(),
            text:    text,
            styles:  styles,
            quit:    false,
        })
    }

    pub fn quit(&self) -> bool {
        self.quit
    }

    pub fn view(&self, stdout: &Stdout) -> io::Result<()> {
        self.text.view(&stdout)
    }

    // return new model based on old model and message
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Resize(y, x) => {
                self.text.resize(x, y);
            }
            Message::Code(c) => {
                match c {
                    UP   => self.text.move_cursor_up(),
                    DOWN => self.text.move_cursor_down(),
                    QUIT => self.quit = true,
                    _ => {}
                }
            }
        }
    }
}
