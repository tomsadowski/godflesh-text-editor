// styles

use ratatui::{
    prelude::*, 
    style::Color, 
    style::Style, 
};



#[derive(Clone, Debug)]
pub struct LineStyles 
{
    pub heading_one:   Style,
    pub heading_two:   Style,
    pub heading_three: Style,
    pub link:          Style,
    pub list_item:     Style,
    pub quote:         Style,
    pub preformat:     Style,
    pub text:          Style,
    pub plaintext:     Style,
}
impl LineStyles 
{
    pub fn new() -> Self 
    {
        let heading_one_style = Style::new()
                .fg(Color::Rgb(208,  96,  96))
                .bg(Color::Rgb( 48,  24,  24))
                .add_modifier(Modifier::BOLD);

        let heading_two_style = Style::new()
                .fg(Color::Rgb(208,  96,  96))
                .bg(Color::Rgb(  0,   0,   0))
                .add_modifier(Modifier::BOLD);

        let heading_three_style = Style::new()
                .fg(Color::Rgb(208,  96,  96))
                .bg(Color::Rgb(  0,   0,   0));

        let link_style = Style::new()
                .fg(Color::Rgb(176,  96, 192))
                .bg(Color::Rgb(  0,   0,   0));

        let text_style = Style::new()
                .fg(Color::Rgb(192, 192, 144))
                .bg(Color::Rgb(  0,   0,   0));

        let list_style = Style::new()
                .fg(Color::Rgb(192, 192, 144))
                .bg(Color::Rgb(  0,   0,   0));

        let quote_style = Style::new()
                .fg(Color::Rgb(192, 192, 144))
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
