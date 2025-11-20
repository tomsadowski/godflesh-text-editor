// model



// *** BEGIN IMPORTS ***
use url::Url;
use std::str::FromStr;
use crate::{
    util, 
    constants,
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
use crossterm::{
    event::{
        self, 
        KeyModifiers, 
        KeyEvent, 
        Event, 
        KeyEventKind, 
        KeyCode},
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
    pub text:    ModelTextType<'a>,
    pub styles:  LineStyles,
    pub x:       u16,
    pub y:       u16,
}
impl<'a> ModelText<'a> 
{
    pub fn plain_text(content: String, styles: &LineStyles) -> Self 
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
            x: 0,
            y: 0
        }
    }

    pub fn init_from_response(status:  Status, 
                              content: String,
                              styles:  &LineStyles) -> Self
    {
        match status 
        {
            Status::InputExpected(variant, msg) => 
            {
                Self::plain_text(content, &styles)
            }
            Status::Success(variant, meta) => 
            {
                if meta.starts_with("text/") 
                {
                    Self
                    {
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
                            x: 0,
                            y: 0,
                    }
                } 
                else 
                {
                    Self::plain_text(format!("no text"), &styles)
                }
            }
            Status::TemporaryFailure(variant, meta) => 
            {
                Self::plain_text(
                    format!("Temporary Failure {:?}: {:?}", variant, meta), 
                    &styles)
            }
            Status::PermanentFailure(variant, meta) => 
            {
                Self::plain_text(
                    format!("Permanent Failure {:?}: {:?}", variant, meta), 
                    &styles)
            }
            Status::Redirect(variant, new_url) => 
            {
                Self::plain_text(
                    format!("Redirect to: {}?", new_url), 
                    &styles)
            }
            Status::ClientCertRequired(variant, meta) => 
            {
                Self::plain_text(
                    format!("Certificate required: {}", meta), 
                    &styles)
            }
        }
    }
}
impl<'a> Widget for &ModelText<'a> 
{
    fn render(self, area: Rect, buf: &mut Buffer) 
    {
        Paragraph::new(self.text.get_lines())
            .wrap(Wrap { trim: true })
            .scroll((self.y, self.x))
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
    pub fn init(_url: &Option<Url>) -> Self 
    {
        let styles = LineStyles::new();

        // return now if no url provided
        let Some(url) = _url else 
        {
            let text = ModelText::plain_text(
                    format!("\twelcome\n\twelcome\n\twelcome"), 
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
            let text = ModelText::plain_text(
                    format!("\n\tdata\n\tretrieval\n\tfailed"), 
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
        let Ok(status) = Status::from_str(&header) else 
        {
            let text = ModelText::plain_text(
                    format!("could not parse status"), 
                    &styles);

            return Self 
            {
                address: address,
                text:    text,
                dialog:  None,
                quit:    false,
            }
        };

        let text = 
            ModelText::init_from_response(status.clone(), content, &styles);

        let dialog = 
            Dialog::init_from_response(status);

        Self 
        {
            address: address,
            text:    text,
            dialog:  dialog,
            quit:    false,
        }
    }
} 
impl<'a> Widget for &Model<'a> 
{
    fn render(self, area: Rect, buf: &mut Buffer) 
    {
        self.text.render(area, buf);
    }
}


pub fn update(model: Model, msg: Message) -> Model 
{
    let mut m = model.clone();
    match msg 
    {
        Message::Stop => 
        { 
            m.quit = true;
        }
        Message::Enter => 
        {
            m.dialog = None;
        }
        Message::Escape => 
        { 
            m.dialog = None;
        }
        Message::Code(c) => 
        {
            if let None = m.dialog 
            {
                match c 
                {
                    constants::LEFT => 
                    {
                        if m.text.x > 0 
                        { 
                            m.text.x = m.text.x - 1;
                        }
                    }
                    constants::UP => 
                    {
                        if m.text.y > 0 
                        { 
                            m.text.y = m.text.y - 1;
                        }
                    }
                    constants::RIGHT => 
                    {
                        m.text.x = m.text.x + 1;
                    }
                    constants::DOWN => 
                    {
                        m.text.y = m.text.y + 1;
                    }
                    _ => {}
                }
            } 
            else 
            {
                m.dialog = Some(Dialog::Message(format!("you pressed {}", c))); 
            }
        }
    }
    // return Model
    m
}


pub fn handle_event(event: event::Event) -> Option<Message> 
{
    // return now if not key event
    let Event::Key(keyevent) = event 
        else {return None};

    match keyevent 
    {
        KeyEvent 
        {
            code: KeyCode::Char('c'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::CONTROL,
            ..
        } => 
        {
            Some(Message::Stop)
        }
        KeyEvent 
        {
            code: KeyCode::Enter,
            kind: KeyEventKind::Press,
            ..
        } => 
        {
            Some(Message::Enter)
        }
        KeyEvent 
        {
            code: KeyCode::Esc,
            kind: KeyEventKind::Press,
            ..
        } => 
        {
            Some(Message::Escape)
        }
        KeyEvent 
        {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            ..
        } => 
        {
            Some(Message::Code(c))
        }
        _ => None
    }
}

