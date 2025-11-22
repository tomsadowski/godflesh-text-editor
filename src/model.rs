// model



// *** BEGIN IMPORTS ***
use url::Url;
use std::str::FromStr;
use crate::{
    util, 
    gemstatus::Status,
    gemtext::{
        GemTextLine,
    },
};
use ratatui::{
    prelude::*, 
    text::{
        Span,
        ToLine,
        Line,
    },
    style::{
        Color, 
        Style, 
    },
    widgets::{
        Paragraph,
        Wrap
    },
};
// *** END IMPORTS ***



#[derive(Clone, Debug)]
pub struct LineStyles 
{
    pub heading_one: Style,
    pub heading_two: Style,
    pub heading_three: Style,
    pub link: Style,
    pub list_item: Style,
    pub quote: Style,
    pub preformat: Style,
    pub text: Style,
    pub plaintext: Style,
}
impl LineStyles 
{
    pub fn new() -> Self 
    {
        let heading_one_style = Style::new()
                .fg(Color::Rgb(112, 160, 192))
                .bg(Color::Rgb( 24,  24,  48))
                .add_modifier(Modifier::BOLD);

        let heading_two_style = Style::new()
                .fg(Color::Rgb(112, 160, 192))
                .bg(Color::Rgb(  0,   0,   0))
                .add_modifier(Modifier::BOLD);

        let heading_three_style = Style::new()
                .fg(Color::Rgb(112, 160, 192))
                .bg(Color::Rgb(  0,   0,   0));

        let link_style = Style::new()
                .fg(Color::Rgb(192, 112, 160))
                .bg(Color::Rgb(  0,   0,   0));

        let text_style = Style::new()
                .fg(Color::Rgb(160, 192, 112))
                .bg(Color::Rgb(  0,   0,   0));

        let list_style = Style::new()
                .fg(Color::Rgb(160, 192, 112))
                .bg(Color::Rgb(  0,   0,   0));

        let quote_style = Style::new()
                .fg(Color::Rgb(160, 192, 112))
                .bg(Color::Rgb(  0,   0,   0));

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
}



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
pub enum Dialog 
{
    AddressBar(Vec<u8>), 
    Prompt(String, Vec<u8>),
    Message(String),
}
impl Dialog 
{
    pub fn init_from_response(status: Status) -> Option<Self> 
    {
        match status 
        {
            Status::InputExpected(variant, msg) => {
                Some(
                    Self::Prompt(
                        format!("input: {}", msg), 
                        vec![]
                    )
                )
            }
            Status::Success(variant, meta) => {
                if meta.starts_with("text/") {
                    None
                } 
                else {
                    Some(
                        Self::Prompt(
                            format!("Download nontext type: {}?", meta), 
                            vec![]
                        )
                    )
                }
            }
            Status::TemporaryFailure(variant, meta) => {
                None
            }
            Status::PermanentFailure(variant, meta) => {
                None
            }
            Status::Redirect(variant, new_url) => {
                Some(
                    Self::Prompt(
                        format!("Redirect to: {}?", new_url), 
                        vec![]
                    )
                )
            }
            Status::ClientCertRequired(variant, meta) => {
                Some(
                    Self::Prompt(
                        format!("Certificate required: {}", meta),
                        vec![]
                    )
                )
            }
        }
    }
}



#[derive(Clone, Debug)]
pub struct GemTextSpan<'a> 
{
    pub source: GemTextLine,
    pub span:   Span<'a>,
}
impl<'a> GemTextSpan<'a> 
{
    fn new(text: &GemTextLine, styles: &LineStyles) -> Self 
    {
        let span = match text.clone() 
        {
            GemTextLine::Text(s) => 
            {
                Span::from(s).style(styles.text)
            }
            GemTextLine::HeadingOne(s) => 
            {
                Span::from(s).style(styles.heading_one)
            }
            GemTextLine::HeadingTwo(s) => 
            {
                Span::from(s).style(styles.heading_two)
            }
            GemTextLine::HeadingThree(s) => 
            {
                Span::from(s).style(styles.heading_three)
            }
            GemTextLine::Link(link) => 
            {
                Span::from(link.get_text()).style(styles.link)
            }
            GemTextLine::Quote(s) => 
            {
                Span::from(s).style(styles.quote)
            }
            GemTextLine::ListItem(s) => 
            {
                Span::from(s).style(styles.list_item)
            }
            GemTextLine::PreFormat(s) => 
            {
                Span::from(s).style(styles.preformat)
            }
        };

        Self 
        {
            source: text.clone(),
            span:   span,
        }
    }

    pub fn get_line(&'a self) -> Line<'a>
    {
        self.span
            .to_line()
            .style(self.span.style)
    }
}



// Implements Widget by parsing ModelText onto a Vec of Spans
#[derive(Clone, Debug)]
pub struct PlainTextSpan<'a> 
{
    pub source: String,
    pub span:   Span<'a>,
}
impl<'a> PlainTextSpan<'a> 
{
    fn new(text: String, styles: &LineStyles) -> Self 
    {
        Self 
        {
            source: text.clone(),
            span:   Span::from(text).style(styles.plaintext),
        }
    }

    pub fn get_line(&'a self) -> Line<'a>
    {
        self.span
            .to_line()
            .style(self.span.style)
    }
}



#[derive(Clone, Debug)]
pub enum ModelTextType<'a>
{
    GemText(Vec<GemTextSpan<'a>>),
    PlainText(Vec<PlainTextSpan<'a>>),
}
impl<'a> ModelTextType<'a> 
{
    pub fn get_lines(&'a self) -> Vec<Line<'a>>
    {
        match &self
        {
            ModelTextType::GemText(vec) => {
                vec
                    .iter()
                    .map(|gemtext| gemtext.get_line())
                    .collect()
            }
            ModelTextType::PlainText(vec) => {
                vec
                    .iter()
                    .map(|plaintext| plaintext.get_line())
                    .collect()
            }
        }
    }
}


#[derive(Clone, Debug)]
pub struct ModelText<'a>
{
    pub text:   ModelTextType<'a>,
    pub styles: LineStyles,
    pub size:   Size,
    pub cursor: Position,
    pub scroll: Position,
}
impl<'a> ModelText<'a> 
{
    pub fn plain_text(content: String, size: Size, styles: &LineStyles) -> Self 
    {
        let vec = content
                .lines()
                .map(
                    |s| PlainTextSpan::new(s.to_string(), &styles))
                .collect();

        let text = ModelTextType::PlainText(vec);

        Self
        {
            text: text,
            styles: styles.clone(),
            size:   size,
            cursor: Position::new(0, 0),
            scroll: Position::new(0, 0),
        }
    }

    pub fn init_from_response(status:  Status, 
                              content: String,
                              size:    Size,
                              styles:  &LineStyles) -> Self
    {
        match status {
            Status::Success(variant, meta) => {
                if meta.starts_with("text/") {
                    Self {
                        text: ModelTextType::GemText(
                                GemTextLine::parse_doc(
                                    content
                                        .lines()
                                        .collect()
                                )
                                .unwrap()
                                .iter()
                                .map(|line| GemTextSpan::new(line, &styles))
                                .collect()
                            ),
                            styles: styles.clone(),
                            size:   size,
                            cursor: Position::new(0, 0),
                            scroll: Position::new(0, 0),
                    }
                } 
                else {
                    Self::plain_text(format!("no text"), size, &styles)
                }
            }
            Status::InputExpected(variant, msg) => {
                Self::plain_text(content, size, &styles)
            }
            Status::TemporaryFailure(variant, meta) => {
                Self::plain_text(
                    format!("Temporary Failure {:?}: {:?}", variant, meta), 
                    size,
                    &styles)
            }
            Status::PermanentFailure(variant, meta) => {
                Self::plain_text(
                    format!("Permanent Failure {:?}: {:?}", variant, meta), 
                    size,
                    &styles)
            }
            Status::Redirect(variant, new_url) => {
                Self::plain_text(
                    format!("Redirect to: {}?", new_url), 
                    size,
                    &styles)
            }
            Status::ClientCertRequired(variant, meta) => {
                Self::plain_text(
                    format!("Certificate required: {}", meta), 
                    size,
                    &styles)
            }
        }
    }

    pub fn move_cursor_up(&mut self) 
    {
        if self.cursor.y > 0 { 
            self.cursor.y -= 1;
        }
        else if self.scroll.y > 0 {
            self.scroll.y -= 1;
        }
    }

    pub fn move_cursor_down(&mut self) 
    {
        if self.cursor.y < self.size.height {
            self.cursor.y += 1;
        }
        else {
            self.scroll.y += 1;
        }
    }

    pub fn move_cursor_left(&mut self) 
    {
        if self.cursor.x > 0 { 
            self.cursor.x -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) 
    {
        self.cursor.x += 1;
    }
}
impl<'a> Widget for &ModelText<'a> 
{
    fn render(self, area: Rect, buf: &mut Buffer) 
    {
        Paragraph::new(self.text.get_lines())
            .wrap(Wrap { trim: true })
            .scroll((self.scroll.y, self.scroll.x))
            .render(area, buf);
    }
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

            return Self 
            {
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

