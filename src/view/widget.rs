// widget

use crate::{
    view::model::Model,
    view::text::ModelText,
    view::dialog::Dialog,
};
use ratatui::{
    prelude::*, 
    widgets::Paragraph,
    widgets::Wrap,
    widgets::Block,
};



impl<'a> Widget for &Model<'a> 
{
    fn render(self, area: Rect, buf: &mut Buffer) 
    {
        if let Some(dialog) = &self.dialog {
            dialog.render(area, buf);
        }
        else {
            self.text.render(area, buf);
        }
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

impl Widget for &Dialog
{
    fn render(self, area: Rect, buf: &mut Buffer) 
    {
        Paragraph::new(self.text.clone())
            .block(Block::bordered().title("Dialog"))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .render(area, buf);

    }
}

