// scr

#[derive(Clone)]
pub struct DataRange {
    pub start: usize, 
    pub end: usize
}
#[derive(Clone, Debug)]
pub struct ScreenRange {
    pub start: u16, 
    pub end: u16
}
impl ScreenRange {
    pub fn from_length(start: u16, len: usize) -> ScreenRange {
        let len = u16::try_from(len).unwrap_or(u16::MIN);
        ScreenRange {start: start, end: start + len}
    }
    // if for some reason a > b, just swap them
    pub fn new(start: u16, end: u16) -> ScreenRange {
        match start > end {
            true =>  ScreenRange {start: end, end: start},
            false => ScreenRange {start: start, end: end},
        }
    }
    pub fn to_data_range(&self) -> DataRange {
        DataRange {start: 0, end: self.len()}
    }
    // index of cursor within its range
    pub fn idx(&self, col: &PosCol) -> usize {
        match col.screen > self.start {
            true => 
                col.data + usize::from(col.screen - self.start),
            false => 
                col.data,
        }

    }
    pub fn len(&self) -> usize {
        usize::from(self.end - self.start)
    }
}

#[derive(Clone, Debug)]
pub struct Screen {
    pub x: u16, 
    pub y: u16,
    pub w: u16, 
    pub h: u16
}
impl Screen {
    pub fn origin(w: u16, h: u16) -> Screen {
        Screen {x: 0, y: 0, w: w, h: h}
    }
    pub fn vcrop(&self, step: u16) -> Screen {
        let screen = self.clone();
        screen.crop_north(step).crop_south(step)
    }
    pub fn hcrop(&self, step: u16) -> Screen {
        let screen = self.clone();
        screen.crop_east(step).crop_west(step)
    }
    pub fn crop_south(&self, step: u16) -> Screen {
        let mut screen = self.clone();
        if step < screen.h {
            screen.h -= step;
        }
        screen
    }
    pub fn crop_east(&self, step: u16) -> Screen {
        let mut screen = self.clone();
        if step < screen.w {
            screen.w -= step;
        }
        screen
    }
    pub fn crop_north(&self, step: u16) -> Screen {
        let mut screen = self.clone();
        if usize::from(step) * 2 < screen.y().len() {
            screen.y += step;
            screen.h -= step;
        }
        screen
    }
    pub fn crop_west(&self, step: u16) -> Screen {
        let mut screen = self.clone();
        if usize::from(step) * 2 < screen.x().len() {
            screen.x += step;
            screen.w -= step;
        }
        screen
    }
    pub fn x(&self) -> ScreenRange {
        ScreenRange {start: self.x, end: self.x + self.w}
    }
    pub fn y(&self) -> ScreenRange {
        ScreenRange {start: self.y, end: self.y + self.h}
    }
}

#[derive(Clone, Debug)]
pub struct Pos {
    pub x: u16, 
    pub y: u16,
    pub i: usize, 
    pub j: usize
}
impl Pos {
    pub fn origin(screen: &Screen) -> Pos {
        Pos {x: screen.x, y: screen.y, i: 0, j: 0}
    }
    pub fn x(&self) -> PosCol {
        PosCol {screen: self.x, data: self.i}
    }
    pub fn y(&self) -> PosCol {
        PosCol {screen: self.y, data: self.j}
    }
}

#[derive(Clone, Debug)]
pub struct PosCol {
    pub screen: u16, 
    pub data: usize
}
impl PosCol {
    pub fn join_with_x(self, x: PosCol) -> Pos {
        Pos {
            x: x.screen, 
            y: self.screen,
            i: x.data, 
            j: self.data,
        }
    }
    pub fn join_with_y(self, y: PosCol) -> Pos {
        Pos {
            x: self.screen, 
            y: y.screen,
            i: self.data, 
            j: y.data,
        }
    }
}
