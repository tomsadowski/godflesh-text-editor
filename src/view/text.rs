// text

use crate::{
    gemini::status::Status,
    gemini::gemtext::GemTextLine,
    gemini::gemtext::GemTextData,
    view::styles::LineStyles,
};
use ratatui::{
    prelude::*, 
    text::Span,
    text::ToLine,
    text::Line,
};



#[derive(Clone, Debug)]
pub struct GemTextSpan<'a> 
{
    pub source: GemTextLine,
    pub span:   Span<'a>,
}
impl<'a> GemTextSpan<'a> 
{
    fn new(line: &GemTextLine, styles: &LineStyles) -> Self 
    {
        let style = match line.data 
        {
            GemTextData::HeadingOne   => styles.heading_one,
            GemTextData::HeadingTwo   => styles.heading_two,
            GemTextData::HeadingThree => styles.heading_three,
            GemTextData::Text         => styles.text,
            GemTextData::Quote        => styles.quote,
            GemTextData::ListItem     => styles.list_item,
            GemTextData::PreFormat    => styles.preformat,
            _                         => styles.link,
        };

        Self {
            source: line.clone(),
            span:   Span::from(line.text.clone()).style(style),
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
pub struct PlainTextSpan<'a> 
{
    pub source: String,
    pub span:   Span<'a>,
}
impl<'a> PlainTextSpan<'a> 
{
    fn new(text: String, styles: &LineStyles) -> Self 
    {
        Self {
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
        match &self {
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

    pub fn get_gemtext_at(&'a self, idx: usize) -> Result<GemTextLine, String> 
    {
        match &self {
            ModelTextType::GemText(vec) => {
                if let Some(gemtext) = vec.get(idx) {
                    return Ok(gemtext.source.clone())
                }
                else {
                    return Err(
                        format!(
                            "expected some gemtext, found none gemtext"))
                }
            }
            ModelTextType::PlainText(vec) => {
                if let Some(plaintext) = vec.get(idx) {
                    return Err(
                        format!(
                            "expected gemtext, found plaintext: {}", 
                            plaintext.source))
                }
                else {
                    return Err(
                        format!(
                            "expected some gemtext, found none plaintext"))
                }
            }
        }
    }
}

// the model's main viewport
#[derive(Clone, Debug)]
pub struct ModelText<'a>
{
    pub text:    ModelTextType<'a>,
    pub styles:  LineStyles,
    pub size:    Size,
    pub cursor:  Position,
    pub scroll:  Position,
    pub vec_idx: usize,
}
impl<'a> ModelText<'a> 
{
    pub fn get_gemtext_under_cursor(&'a self) -> Result<GemTextLine, String>
    {
        self.text.get_gemtext_at(self.vec_idx)
    }

    pub fn plain_text(content: String, size: Size, styles: &LineStyles) -> Self 
    {
        let vec = content
                .lines()
                .map(
                    |s| PlainTextSpan::new(s.to_string(), &styles))
                .collect();

        let text = ModelTextType::PlainText(vec);

        Self {
            text:    text,
            styles:  styles.clone(),
            size:    size,
            cursor:  Position::new(0, 0),
            scroll:  Position::new(0, 0),
            vec_idx: 0,
        }
    }

    pub fn gemtext(content: String, size: Size, styles: &LineStyles) -> Self 
    {
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
            styles:  styles.clone(),
            size:    size,
            cursor:  Position::new(0, 0),
            scroll:  Position::new(0, 0),
            vec_idx: 0,
        }
    }

    pub fn update_from_response(self, status: Status, content: String) -> Self
    {
        Self::init_from_response(status, content, self.size, &self.styles)
    }

    pub fn init_from_response(status:  Status, 
                              content: String,
                              size:    Size,
                              styles:  &LineStyles) -> Self
    {
        match status {
            Status::Success(_variant, meta) => {
                if meta.starts_with("text/") {
                    Self::gemtext(content, size, &styles)
                } 
                else {
                    Self::plain_text(format!("no text"), size, &styles)
                }
            }
            Status::InputExpected(_variant, _msg) => {
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
            Status::Redirect(_variant, new_url) => {
                Self::plain_text(
                    format!("Redirect to: {}?", new_url), 
                    size,
                    &styles)
            }
            Status::ClientCertRequired(_variant, meta) => {
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
            self.vec_idx  -= 1;
        }
        else if self.scroll.y > 0 {
            self.scroll.y -= 1;
            self.vec_idx  -= 1;
        }
    }

    pub fn move_cursor_down(&mut self) 
    {
        if self.cursor.y < self.size.height {
            self.cursor.y += 1;
            self.vec_idx  += 1;
        }
        else {
            self.scroll.y += 1;
            self.vec_idx  += 1;
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
