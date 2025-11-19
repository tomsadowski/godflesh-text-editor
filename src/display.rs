// display



// *** BEGIN IMPORTS ***
use url::Url;
use crate::{
    model::{
        Model, 
        ModelText
    },
    gemtext::{
        GemTextLine,
    }
};
use ratatui::{
    prelude::*, 
    text::{
        Line,
        Span,
        ToLine,
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
                .fg(Color::Rgb(128, 64, 32))
                .bg(Color::Rgb(16,  32, 32));
        let heading_two_style = Style::new()
                .fg(Color::Rgb(128, 64, 32))
                .bg(Color::Rgb(16,  32, 32));
        let heading_three_style = Style::new()
                .fg(Color::Rgb(128, 64, 32))
                .bg(Color::Rgb(16,  32, 32));
        let link_style = Style::new()
                .fg(Color::Rgb(128, 64, 32))
                .bg(Color::Rgb(16,  32, 32));
        let text_style = Style::new()
                .fg(Color::Rgb(128, 64, 32))
                .bg(Color::Rgb(16,  32, 32));
        let list_style = Style::new()
                .fg(Color::Rgb(128, 64, 32))
                .bg(Color::Rgb(16,  32, 32));
        let quote_style = Style::new()
                .fg(Color::Rgb(128, 64, 32))
                .bg(Color::Rgb(16,  32, 32));

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


// Implements Widget by parsing ModelText onto a Vec of Spans
#[derive(Clone, Debug)]
pub struct GemTextSpan<'a> 
{
    pub source: GemTextLine,
    pub span:   Span<'a>,
}

impl<'a> GemTextSpan<'a> 
{
    fn new(text: &GemTextLine, styles: LineStyles) -> Self 
    {
        let span = match text.clone() 
        {
            GemTextLine::Text(s) => {
                Span::from(s).style(styles.text)
            }
            GemTextLine::HeadingOne(s) => {
                Span::from(s).style(styles.heading_one)
            }
            GemTextLine::HeadingTwo(s) => {
                Span::from(s).style(styles.heading_two)
            }
            GemTextLine::HeadingThree(s) => {
                Span::from(s).style(styles.heading_three)
            }
            GemTextLine::Link(link) => {
                Span::from(link.get_text()).style(styles.link)
            }
            GemTextLine::Quote(s) => {
                Span::from(s).style(styles.quote)
            }
            GemTextLine::ListItem(s) => {
                Span::from(s).style(styles.list_item)
            }
            GemTextLine::PreFormat(s) => {
                Span::from(s).style(styles.preformat)
            }
        };
        Self {
            source: text.clone(),
            span:   span,
        }
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
    fn new(text: &'a str, styles: LineStyles) -> Self 
    {
        Self {
            source: String::from(text),
            span:   Span::from(text).style(styles.plaintext),
        }
    }
}


// Implements Widget by parsing ModelText onto a Vec of Spans
#[derive(Clone, Debug)]
pub enum DisplayModelText<'a> 
{
    GemText(Vec<GemTextSpan<'a>>),
    PlainText(Vec<PlainTextSpan<'a>>),
}

impl<'a> DisplayModelText<'a> 
{
    pub fn new(text: ModelText, styles: LineStyles) -> Self 
    {
        let styles = styles.clone();
        match text {
            ModelText::GemText(lines) => 
                Self::GemText(
                    lines
                        .iter()
                        .map(|line| GemTextSpan::new(line, styles))
                        .collect()
                ),
            ModelText::PlainText(lines) => 
                Self::PlainText(
                    lines
                        .iter()
                        .map(|line| PlainTextSpan::new(line, styles))
                        .collect()
                ),
        }
    }
}

impl<'a> Widget for &DisplayModelText<'a> 
{
    fn render(self, area: Rect, buf: &mut Buffer) 
    {
        let lines = match self 
        {
            DisplayModelText::GemText(vec) => {
                vec
                    .iter()
                    .map(|gemtext| 
                        gemtext.span
                            .to_line()
                            .style(gemtext.span.style))
                    .collect()
            }
            DisplayModelText::PlainText(vec) => {
                vec
                    .iter()
                    .map(|plaintext| 
                        plaintext.span
                            .to_line()
                            .style(plaintext.span.style))
                    .collect()
            }
        };

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .scroll((self.y, self.x))
            .render(area, buf);
    }
}


// Implements Widget by projecting Model onto Widgets
#[derive(Clone, Debug)]
pub struct DisplayModel<'a> 
{
    pub text:   DisplayModelText<'a>,
    pub styles: LineStyles,
    pub x: u16,
    pub y: u16,
}

impl<'a> DisplayModel<'a> 
{
    pub fn init(model: &Model) -> Self 
    {
        let text   = source.text.clone();
        let styles = LineStyles::new();
        Self 
        {
            source: model,
            styles: styles.clone(),
            text:   DisplayModelText::new(text, styles),
            x: 0, 
            y: 0,
        }
    }

    pub fn update(mut self, model: &'a Model) 
    {
        self.source = model.clone();
        self.text   = DisplayModelText::new(&model.text, self.styles);
    }
}

impl<'a> Widget for &DisplayModel<'a> 
{
    fn render(self, area: Rect, buf: &mut Buffer) 
    {
        self.text.render(area, buf);
    }
}
