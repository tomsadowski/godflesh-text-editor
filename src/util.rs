// util

// a rectangle specified by a point and some lengths
#[derive(Clone, Debug)]
pub struct Rect {
    pub x: u16, 
    pub y: u16, 
    pub w: u16, 
    pub h: u16,
}
#[derive(Clone, Debug)]
pub struct Cursor {
    pub cur: u16,
    pub min: u16,
    pub max: u16,
}
impl Cursor {
    // sets limits given length of text and rect
    pub fn new(len: usize, rect: &Rect) -> Self {
        let len = match u16::try_from(len) {
            Ok(t) => t, _ => u16::MAX,
        };
        Self {
            min: rect.y,
            cur: rect.y, 
            max: std::cmp::min(len, rect.h),
        }
    }
    pub fn resize(&mut self, len: usize, rect: &Rect) {
        let len = match u16::try_from(len) {
            Ok(t) => t, _ => u16::MAX,
        };
        self.min = rect.y;
        self.max = std::cmp::min(len, rect.h);
        self.cur = (self.min + self.max - 1) / 2;
    }
    pub fn move_up(&mut self, step: u16) -> bool {
        if (self.min + step) <= self.cur {
            self.cur -= step; 
            true
        } else { false }
    }
    pub fn move_down(&mut self, step: u16) -> bool {
        if (self.cur + step) <= (self.min + self.max - 1) {
            self.cur += step; 
            true 
        } else { false }
    }
    // index of cursor within its rect
    pub fn get_index(&self) -> usize {
        usize::from(self.cur - self.min)
    }
}
// scroll over data when cursor position is at a limit
// defined by rect
#[derive(Clone, Debug)]
pub struct ScrollingCursor {
    buf:              u16,
    pub inner_cursor: Cursor,
    pub outer_cursor: Cursor,
    pub scroll:       usize,
    pub maxscroll:    usize,
}
impl ScrollingCursor {
    // sets limits given length of text and rect
    pub fn new(len: usize, rect: &Rect, buf: u16) -> Self {
        let outer_cursor = Cursor::new(len, rect);
        let inner_cursor = Cursor::new(len, 
                &Rect {
                    x: rect.x, 
                    y: rect.y + buf, 
                    w: rect.w, 
                    h: rect.h - buf});
        Self {
            buf:          buf,
            scroll:       0, 
            maxscroll:    len - usize::from(outer_cursor.max),
            outer_cursor: outer_cursor,
            inner_cursor: inner_cursor,
        }
    }
    // like Self::new but tries to preserve scroll
    pub fn resize(&mut self, len: usize, rect: &Rect) {
        self.outer_cursor.resize(len, rect);
        self.inner_cursor.resize(len, 
            &Rect {
                x: rect.x, 
                y: rect.y + self.buf, 
                w: rect.w, 
                h: rect.h - self.buf});
        self.maxscroll = len - usize::from(self.outer_cursor.max);
        self.scroll    = std::cmp::min(self.scroll, self.maxscroll);
    }
    // scroll up when cursor is at highest position
    pub fn move_up(&mut self, step: u16) -> bool {
        let scrollstep = usize::from(step);
        if self.inner_cursor.move_up(step) {
            self.outer_cursor.move_up(step);
            true
        } else if usize::MIN + scrollstep <= self.scroll {
            self.scroll -= scrollstep; 
            true
        } else if self.outer_cursor.move_up(step) {
            true
        } else { false }
    }
    // scroll down when cursor is at lowest position
    pub fn move_down(&mut self, step: u16) -> bool {
        let scrollstep = usize::from(step);
        if self.inner_cursor.move_down(step) {
            self.outer_cursor.move_down(step);
            true 
        } else if (self.scroll + scrollstep) <= self.maxscroll {
            self.scroll += scrollstep; 
            true
        } else if self.outer_cursor.move_down(step) {
            true
        }
        else { false }
    }
    // index of cursor within its rect
    pub fn get_cursor(&self) -> u16 {
        self.outer_cursor.cur
    }
    // index of cursor within its rect
    pub fn get_scroll(&self) -> usize {
        self.scroll
    }
    pub fn get_screen_start(&self) -> u16 {
        self.outer_cursor.min
    }
    // index of cursor within its rect
    pub fn get_index(&self) -> usize {
        self.scroll + self.outer_cursor.get_index()
    }
    // returns the start and end of displayable text
    pub fn get_display_range(&self) -> (usize, usize) {
        (self.scroll, self.scroll + usize::from(self.outer_cursor.max))
    }
}
// call wrap for each element in the list
pub fn wrap_list(lines: &Vec<String>, w: u16) 
    -> Vec<(usize, String)> 
{
    let mut display: Vec<(usize, String)> = vec![];
    for (i, l) in lines.iter().enumerate() {
        let v = wrap(l, w);
        for s in v.iter() {
            display.push((i, s.to_string()));
        }
    }
    display
}
// wrap text in terminal
pub fn wrap(line: &str, screenwidth: u16) -> Vec<String> {
    let width = usize::from(screenwidth);
    let length = line.len();
    let mut wrapped: Vec<String> = vec![];
    // assume slice bounds
    let mut start = 0;
    let mut end = width;
    while end < length {
        start = line.ceil_char_boundary(start);
        end = line.floor_char_boundary(end);
        let longest = &line[start..end];
        // try to break line at a space
        match longest.rsplit_once(' ') {
            // there is a space to break on
            Some((a, b)) => {
                let shortest = match a.len() {
                    0 => b,
                    _ => a,
                };
                wrapped.push(String::from(shortest.trim()));
                start += shortest.len();
                end = start + width;
            }
            // there is no space to break on
            None => {
                wrapped.push(String::from(longest.trim()));
                start = end;
                end += width;
            }
        }
    }
    // add the remaining text
    if start < length {
        start = line.floor_char_boundary(start);
        wrapped.push(String::from(line[start..].trim()));
    }
    wrapped
}
pub fn split_whitespace_once(source: &str) -> (&str, &str) {
    let line = source.trim();
    let (a, b) = {
        if let Some(i) = line.find("\u{0009}") {
            (line[..i].trim(), line[i..].trim())
        } else if let Some(i) = line.find(" ") {
            (line[..i].trim(), line[i..].trim())
        } else {
            (line, line)
        }
    };
    (a, b)
}
