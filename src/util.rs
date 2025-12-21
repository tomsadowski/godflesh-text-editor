// util

use std::{time::Duration}; 
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use native_tls::TlsConnector;
use tempfile::NamedTempFile;
use crossterm::style::{Colors};

pub trait GetColors {
    fn getcolors(&self) -> Colors;
}
pub fn get_data(url: &url::Url) -> Result<(String, String), String> {
    let host = url.host_str().unwrap_or("");
    let urlf = format!("{}:1965", host);
    let failmsg = "Could not connect to ";

    // get connector
    let connector = TlsConnector::builder()
        .danger_accept_invalid_hostnames(true)
        .danger_accept_invalid_certs(true)
        .build()
        .or_else(|e| Err(format!("{}{}\n{}", failmsg, urlf, e)))?;

    // get socket address iterator
    let mut addrs_iter = urlf.to_socket_addrs()
        .or_else(|e| Err(format!("{}{}\n{}", failmsg, urlf, e)))?;

    // get socket address from socket address iterator
    let Some(socket_addr) = addrs_iter.next() 
        else {return Err(format!("Could not connect to {}", urlf))};

    // get tcp stream from socket address
    let tcpstream = TcpStream::connect_timeout
        (&socket_addr, Duration::new(10, 0))
        .or_else(|e| Err(format!("Could not connect to {}\n{}", urlf, e)))?;

    // get stream from tcp stream
    let mut stream = connector.connect(&host, tcpstream) 
        .or_else(|e| Err(format!("Could not connect to {}\n{}", urlf, e)))?;

    // write url to stream
    stream.write_all(format!("{}\r\n", url).as_bytes())
        .or_else(|e| Err(format!("Could not write to {}\n{}", url, e)))?;

    // initialize response vector
    let mut response = vec![];

    // load response vector from stream
    stream.read_to_end(&mut response)
        .or_else(|e| Err(format!("Could not read {}\n{}", url, e)))?;

    // find clrf in response vector
    let Some(clrf_idx) = find_clrf(&response) 
        else {return Err("Could not find the clrf".to_string())};

    // separate response from content
    let content = response.split_off(clrf_idx + 2);

    // convert to String
    let content  = String::from_utf8_lossy(&content).to_string();
    let response = String::from_utf8_lossy(&response).to_string();
    Ok((response, content))
}
pub fn download(content: String) {
    let path = write_tmp_file(content.into_bytes());
    open::that(path).unwrap();
}
fn write_tmp_file(content: Vec<u8>) -> std::path::PathBuf {
    let mut tmp_file = NamedTempFile::new().unwrap();
    tmp_file.write_all(&content).unwrap();
    let (_file, path) = tmp_file.keep().unwrap();
    path
}
fn find_clrf(data: &[u8]) -> Option<usize> {
    let clrf = b"\r\n";
    data.windows(clrf.len()).position(|window| window == clrf)
}
// View currently in use
#[derive(Clone, Debug)]
pub enum View {
    Tab,
    History,
    Bookmarks,
    Quit,
}
// Message returned from a view's update method
#[derive(Clone, Debug)]
pub enum ViewMsg {
    None,
    Go(String),
    Switch(View),
}
// a rectangle specified by a point and some lengths
#[derive(Clone, Debug)]
pub struct Rect {
    pub x: u16, pub y: u16, pub w: u16, pub h: u16,
}
impl Rect {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self {x: x, y: y, w: w, h: h}
    }
}
// cursor that scrolls over data when it can't move
#[derive(Clone, Debug)]
pub struct ScrollingCursor {
    pub scroll: usize,
    pub maxscroll: usize,
    pub cursor: u16,
    pub rect: Rect,
}
impl ScrollingCursor {
    // sets limits given length of text and bounding box
    pub fn new(textlength: usize, rect: &Rect) -> Self {
        let len = match u16::try_from(textlength) {
            Ok(t) => t, _ => u16::MAX,
        };
        match len < rect.h {
            // no scrolling allowed
            true => Self {
                rect: Rect::new(rect.x, rect.y, rect.w, len),
                cursor: rect.y, 
                scroll: 0, 
                maxscroll: 0,
            },
            // scrolling allowed
            false => Self {
                rect: rect.clone(),
                cursor: rect.y, 
                scroll: 0, 
                maxscroll: textlength - usize::from(rect.h),
            },
        }
    }
    // like Self::new method but tries to preserve scroll
    pub fn resize(&mut self, textlength: usize, rect: &Rect) {
        let len = match u16::try_from(textlength) {
            Ok(t) => t, _ => u16::MAX,
        };
        match len < rect.h {
            // no scrolling allowed
            true => {
                self.rect = Rect::new(rect.x, rect.y, rect.w, len);
                self.scroll = 0;
                self.maxscroll = 0;
            },
            // scrolling allowed
            false => {
                self.rect = rect.clone();
                self.scroll = std::cmp::min(self.scroll, self.maxscroll);
                self.maxscroll = textlength - usize::from(rect.h);
            },
        }
        self.cursor = (self.rect.y + self.rect.h - 1) / 2;
    }
    // scroll up when cursor is at highest position
    pub fn moveup(&mut self, step: u16) -> bool {
        let scrollstep = usize::from(step);
        if (self.rect.y + step) <= self.cursor {
            self.cursor -= step;
            true
        } else if usize::MIN + scrollstep <= self.scroll {
            self.scroll -= scrollstep;
            true
        } else {
            false
        }
    }
    // scroll down when cursor is at lowest position
    pub fn movedown(&mut self, step: u16) -> bool {
        let scrollstep = usize::from(step);
        if (self.cursor + step) <= (self.rect.y + self.rect.h - 1) {
            self.cursor += step;
            true 
        } else if (self.scroll + scrollstep) <= self.maxscroll {
            self.scroll += scrollstep;
            true
        } else {
            false
        }
    }
    // returns the start and end of displayable text
    pub fn slicebounds(&self) -> (usize, usize) {
        let start = self.scroll;
        let end = self.scroll + usize::from(self.rect.h);
        (start, end)
    }
    // index of cursor within its bounding box
    pub fn index(&self) -> usize {
        usize::from(self.cursor - self.rect.y)
    }
}
// wrap text in terminal
pub fn wrap(line: &str, screenwidth: u16) -> Vec<String> {
    let width = usize::from(screenwidth);
    let length = line.len();
    let mut wrapped: Vec<String> = vec![];
    // assume slice bounds
    let (mut start, mut end) = (0, width);
    while end < length {
        let longest = &line[start..end];
        // try to break line at a space
        match longest.rsplit_once(' ') {
            // there is a space to break on
            Some((a, b)) => {
                let shortest = match a.len() {
                    0 => b,
                    _ => a,
                };
                wrapped.push(String::from(shortest));
                start += shortest.len();
                end = start + width;
            }
            // there is no space to break on
            None => {
                wrapped.push(String::from(longest));
                start = end;
                end += width;
            }
        }
    }
    // add the remaining text
    if start < length {
        wrapped.push(String::from(&line[start..length]));
    }
    wrapped
}
// cut text in terminal, adding "..." to indicate that it 
// continues beyond the screen
pub fn cut(line: &str, screenwidth: u16) -> String {
    let mut width = usize::from(screenwidth);
    if line.len() < width {
        return String::from(line)
    } else {
        width -= 2;
        let longest = &line[..width];
        match longest.rsplit_once(' ') {
            Some((a, b)) => {
                let shortest = match a.len() {
                    0 => b,
                    _ => a,
                };
                return format!("{}..", shortest)
            }
            None => {
                return format!("{}..", longest)
            }
        }

    }
}
// call cut for each element in the list
pub fn cutlist<T>(lines: &Vec<(T, String)>, w: u16) -> Vec<(usize, String)> {
    let mut display: Vec<(usize, String)> = vec![];
    for (i, (_, l)) in lines.iter().enumerate() {
        display.push((i, cut(l, w)));
    }
    display
}
// call wrap for each element in the list
pub fn wraplist<T>(lines: &Vec<(T, String)>, w: u16) -> Vec<(usize, String)> {
    let mut display: Vec<(usize, String)> = vec![];
    for (i, (_, l)) in lines.iter().enumerate() {
        let v = wrap(l, w);
        for s in v.iter() {
            display.push((i, s.to_string()));
        }
    }
    display
}
