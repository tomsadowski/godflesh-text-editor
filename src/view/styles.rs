// styles

use crossterm::{
    style::Color,
    style::Colors,
};
use crate::gemini::gemtext::GemTextData;


#[derive(Clone, Debug)]
pub struct LineStyles 
{
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
impl LineStyles 
{
    pub fn new() -> Self 
    {
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

    pub fn get_colors(&self, data: GemTextData) -> Colors {
        match data {
            GemTextData::HeadingOne   => self.heading_one,
            GemTextData::HeadingTwo   => self.heading_two,
            GemTextData::HeadingThree => self.heading_three,
            GemTextData::Text         => self.text,
            GemTextData::Quote        => self.quote,
            GemTextData::ListItem     => self.list_item,
            GemTextData::PreFormat    => self.preformat,
            _                         => self.link,
        }
    }
}
