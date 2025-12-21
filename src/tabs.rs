// pager/src/tabs

use crate::util::{self, Rect};
use crate::gemtext::{self, Scheme, GemTextData};
use crate::widget::{Selector, Dialog, InputType, DialogMsg};
use crate::util::{ViewMsg};
use crossterm::{QueueableCommand, cursor, terminal};
use crossterm::event::{KeyCode};
use crossterm::style::{self, Colors, Color};
use std::io::{self, Stdout};
use url::Url;

#[derive(Clone, Debug)]
pub struct TabMgr {
    rect: Rect,
    tabs: Vec<Tab>,
    // index of current tab
    curindex: usize,
    // meta data to display at all times
    bannerstr: String,
    bannerstrcolor: Colors,
    // separate banner from page
    bannerline: String,
    bannerlinecolor: Colors,
}
impl TabMgr {
    pub fn new(rect: &Rect, p: &str) -> Self {
        let rect = Rect::new(rect.x, rect.y + 2, rect.w, rect.h - 1);
        let url = Url::parse(p).unwrap();
        Self {
            rect: rect.clone(),
            tabs: vec![Tab::new(&rect, &url)],
            curindex: 0,
            bannerstr: Self::bannerstr(0, 1, &url),
            bannerline: Self::bannerline(rect.w),
            bannerstrcolor: Colors::new(
                Color::Rgb {r: 180, g: 180, b: 180},
                Color::Rgb {r: 0, g: 0, b: 0}),
            bannerlinecolor: Colors::new(
                Color::Rgb {r: 180, g: 180, b: 180},
                Color::Rgb {r: 0, g: 0, b: 0}),
        }
    }
    // adjust length of banner line, resize all tabs
    pub fn resize(&mut self, rect: &Rect) {
        self.rect = Rect::new(rect.x, rect.y + 2, rect.w, rect.h - 1);
        self.bannerline = Self::bannerline(rect.w);
        for d in self.tabs.iter_mut() {
            d.resize(&self.rect);
        }
    }
    // display banner and page
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?
            .queue(style::SetColors(self.bannerstrcolor))?
            .queue(style::Print(self.bannerstr.as_str()))?
            .queue(cursor::MoveTo(0, 1))?
            .queue(style::SetColors(self.bannerlinecolor))?
            .queue(style::Print(&self.bannerline))?;
        self.tabs[self.curindex].view(stdout)
    }
    // send keycode to current tab and process response
    pub fn update(&mut self, keycode: &KeyCode) -> bool {
        match self.tabs[self.curindex].update(keycode) {
            Some(msg) => {
                match msg {
                    TabMsg::Msg(ViewMsg::Go(p)) => {
                        let url = Url::parse(p.as_str()).unwrap();
                        self.tabs.push(Tab::new(&self.rect, &url));
                        self.curindex = self.tabs.len() - 1;
                    }
                    TabMsg::DeleteMe => {
                        if self.tabs.len() > 1 {
                            self.tabs.remove(self.curindex);
                            self.curindex = self.tabs.len() - 1;
                        }
                    }
                    TabMsg::CycleLeft => {
                        match self.curindex == 0 {
                            true => self.curindex = self.tabs.len() - 1,
                            false => self.curindex -= 1,
                        }
                    }
                    TabMsg::CycleRight => {
                        match self.curindex == self.tabs.len() - 1 {
                            true => self.curindex = 0,
                            false => self.curindex += 1,
                        }
                    }
                    _ => {},
                }
                let len = self.tabs.len();
                let path = &self.tabs[self.curindex].path;
                self.bannerstr = Self::bannerstr(self.curindex, len, path);
                self.bannerline = Self::bannerline(self.rect.w);
                true
            }
            None => false,
        }
    }
    fn bannerstr(curindex: usize, totaltab: usize, path: &Url) -> String {
        format!("{}/{}: {}", curindex + 1, totaltab, path)
    }
    fn bannerline(w: u16) -> String {
        String::from("-").repeat(usize::from(w))
    }
}

#[derive(Clone, Debug)]
pub enum Action {
    None,
    GoTo,
    DeleteMe,
    Go(String),
}
#[derive(Clone, Debug)]
pub enum TabMsg {
    CycleLeft,
    CycleRight,
    DeleteMe,
    Go(String),
    Msg(ViewMsg),
}
#[derive(Clone, Debug)]
pub struct Tab {
    rect: Rect,
    pub path: Url,
    dlgstack: Vec<Dialog<Action>>,
    page: Selector<GemTextData>,
}
impl Tab {
    pub fn new(rect: &Rect, path: &Url) -> Self {
        let src = util::get_data(path).unwrap();
        let text = gemtext::parse_doc(src.1.lines().collect()).unwrap();
        Self {
            rect: rect.clone(),
            path: path.clone(),
            dlgstack: vec![],
            page: Selector::new(rect, text, true),
        }
    }
    // show dialog if there's a dialog, otherwise show page
    pub fn view(&self, stdout: &Stdout) -> io::Result<()> {
        match self.dlgstack.last() {
            Some(d) => d.view(stdout),
            _ => self.page.view(stdout),
        }
    }
    // resize page and all dialogs
    pub fn resize(&mut self, rect: &Rect) {
        self.rect = rect.clone();
        self.page.resize(&rect);
        for d in self.dlgstack.iter_mut() {
            d.resize(&rect);
        }
    }
    pub fn update(&mut self, keycode: &KeyCode) -> Option<TabMsg> {
        // send keycode to dialog if there is a dialog
        if let Some(d) = self.dlgstack.last_mut() {
            match d.update(keycode) {
                Some(DialogMsg::Submit) => {
                    let msg = match (&d.action, &d.input) {
                        (Action::Go(p), InputType::Choose((c, _))) => {
                            match c {
                                'y' => 
                                    Some(TabMsg::Msg(ViewMsg::Go(p.clone()))),
                                _ => 
                                    Some(TabMsg::Msg(ViewMsg::None)),
                            }
                        }
                        (Action::GoTo, InputType::Input(v)) => {
                            Some(TabMsg::Msg(ViewMsg::Go(v.clone())))
                        }
                        (Action::DeleteMe, InputType::Choose((c, _))) => {
                            match c {
                                'y' => Some(TabMsg::DeleteMe),
                                _ => Some(TabMsg::Msg(ViewMsg::None)),
                            }
                        }
                        (_, _) => 
                            Some(TabMsg::Msg(ViewMsg::None)),
                    };
                    self.dlgstack.pop();
                    return msg
                }
                Some(DialogMsg::Cancel) => {
                    self.dlgstack.pop();
                    return Some(TabMsg::Msg(ViewMsg::None))
                }
                Some(_) => {
                    return Some(TabMsg::Msg(ViewMsg::None))
                }
               _ => return None
            }
        }
        // there is no dialog, process keycode here
        match keycode {
            KeyCode::Char('v') => {
                let dialog = Dialog::new(
                    &self.rect,
                    Action::DeleteMe,
                    InputType::Choose(('n', vec![
                        ('y', String::from("yes")), 
                        ('n', String::from("no"))])),
                    "Delete current tab?");
                self.dlgstack.push(dialog);
                Some(TabMsg::Msg(ViewMsg::None))
            }
            KeyCode::Char('p') => {
                let dialog = Dialog::new(
                    &self.rect,
                    Action::GoTo,
                    InputType::Input(String::from("")),
                    "enter path: ");
                self.dlgstack.push(dialog);
                Some(TabMsg::Msg(ViewMsg::None))
            }
            KeyCode::Char('i') => {
                self.page.cursor.movedown(1);
                Some(TabMsg::Msg(ViewMsg::None))
            }
            KeyCode::Char('o') => {
                self.page.cursor.moveup(1);
                Some(TabMsg::Msg(ViewMsg::None))
            }
            KeyCode::Char('e') => {
                Some(TabMsg::CycleLeft)
            }
            KeyCode::Char('n') => {
                Some(TabMsg::CycleRight)
            }
            // make a dialog
            KeyCode::Enter => {
                let dialog = match self.page.selectundercursor() {
                    GemTextData::Link(Scheme::Relative(l)) => Dialog::new(
                        &self.rect,
                        Action::Go(self.path.join(l).unwrap().to_string()),
                        InputType::Choose(('n', vec![
                            ('y', String::from("yes")), 
                            ('n', String::from("no"))])),
                        &format!("go to {}?", l)),
                    GemTextData::Link(Scheme::Gemini(l)) => Dialog::new(
                        &self.rect,
                        Action::Go(l.to_string()),
                        InputType::Choose(('n', vec![
                            ('y', String::from("yes")), 
                            ('n', String::from("no"))])),
                        &format!("go to {}?", l)),
                    _ => Dialog::new(
                        &self.rect,
                        Action::None,
                        InputType::None,
                        "You've selected some text. "),
                };
                self.dlgstack.push(dialog);
                Some(TabMsg::Msg(ViewMsg::None))
            }
            _ => None,
        }
    }
}
