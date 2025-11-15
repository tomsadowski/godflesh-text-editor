// widget

// *** BEGIN IMPORTS ***
use crate::{
    model
};
use ratatui::{
    prelude::*, 
    text::{
        Line,
    },
    widgets::{
        Paragraph,
        Wrap
    },
};
// *** END IMPORTS ***

#[derive(Clone, Debug)]
pub struct ModelWidget<'a> {
    pub source:  model::Model,
    pub dialog:  Vec<Line<'a>>,
    pub address: Line<'a>,
    pub text:    Vec<Line<'a>>,
}
impl<'a> ModelWidget<'a> {
    pub fn new(model: model::Model) -> Self {
        Self {
            source:  model,
            dialog:  vec![],
            address: Line::default(),
            text:    vec![],
        }
    }
}
impl<'a> Widget for &ModelWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = format!("{:#?}", self);
        let p = Paragraph::new(text)
            .wrap(Wrap { trim: true });
        p.render(area, buf);
    }
}
