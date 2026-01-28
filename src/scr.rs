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
    // if for some reason a > b, just swap them
    pub fn new(start: u16, end: u16) -> ScreenRange {
        match start > end {
            true =>  ScreenRange {start: end, end: start},
            false => ScreenRange {start: start, end: end},
        }
    }
    pub fn from_length(start: u16, len: usize) -> ScreenRange {
        let len = u16::try_from(len).unwrap_or(u16::MIN);
        ScreenRange {start: start, end: start + len}
    }
    pub fn to_data_range(&self) -> DataRange {
        DataRange {start: 0, end: self.len()}
    }
    // index of cursor within its range
    pub fn idx(&self, col: &PosCol) -> usize {
        if col.screen > self.start {
            col.data + usize::from(col.screen - self.start)
        } else {
            col.data
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
    pub fn ycrop(&self, step: u16) -> Screen {
        let screen = self.clone();
        screen.crop_north(step).crop_south(step)
    }
    pub fn xcrop(&self, step: u16) -> Screen {
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
pub struct DataScreen {
    pub outer: Screen,
    pub inner: Screen,
}
impl DataScreen {
    pub fn new(outer: Screen, x: u16, y: u16) -> DataScreen {
        Self {
            inner: outer.xcrop(x).ycrop(y),
            outer: outer,
        }
    }
    pub fn new_pos(&self) -> Pos {
        Pos::origin(&self.outer)
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

pub fn move_into(   
                    inner: &ScreenRange, 
                    outer: &ScreenRange, 
                    pos: &PosCol,
                    len: usize  
                    ) -> PosCol 
{
    let mut pos = pos.clone();
    let (start, end) = 
        match len < outer.len() {
            true => {
                pos.data = 0;
                (outer.start, 
                 outer.start + u16::try_from(len)
                    .unwrap_or(u16::MIN))
            }
            false => 
                (outer.start, inner.end)
        };
    if pos.screen < start {
        pos.screen = start;
    }
    else if pos.screen >= end {
        pos.screen = end;
    }
    pos
}

pub fn move_backward(   
                        inner:  &ScreenRange, 
                        outer:  &ScreenRange, 
                        pos:    &PosCol, 
                        step:   u16 
                        ) -> Option<PosCol> 
{
    let mut step = step;
    let mut pos = pos.clone();
    match (pos.screen == outer.start, pos.data == usize::MIN) {
        // nowhere to go, nothing to change
        (true, true) => {
            None
        }
        // move data point
        (true, false) => {
            if usize::from(step) < pos.data  {
                pos.data -= usize::from(step);
            } else {
                pos.data = usize::MIN;
            }
            Some(pos)
        }
        // move screen point
        (false, true) => {
            if outer.start + step <= pos.screen {
                pos.screen -= step;
            } else {
                pos.screen = outer.start;
            }
            Some(pos)
        }
        (false, false) => {
            if inner.start + step <= pos.screen {
                pos.screen -= step;
                Some(pos)
            } else if inner.start == pos.screen {
                if usize::from(step) <= pos.data {
                    pos.data -= usize::from(step);
                    Some(pos)
                } else {
                    step -= u16::try_from(pos.data).unwrap_or(u16::MIN);
                    pos.data = usize::MIN;
                    move_backward(inner, outer, &pos, step).or(Some(pos))
                }
            } else {
                step -= pos.screen - inner.start;
                pos.screen = inner.start;
                move_backward(inner, outer, &pos, step).or(Some(pos))
            }
        }
    }
}

pub fn move_forward(    
                        inner:      &ScreenRange, 
                        outer:      &ScreenRange, 
                        pos:        &PosCol, 
                        dlength:    usize,
                        step:       u16 
                        ) -> Option<PosCol> 
{
    let mut step = step;
    let mut pos = pos.clone();
    let screen_len = outer.len();
    let screen_data_end = 
        u16::try_from(
            std::cmp::min(
                    usize::from(outer.start) + dlength, 
                    usize::from(outer.end)
                ))
        .unwrap_or(u16::MIN);
    let max_data = dlength.saturating_sub(screen_len);

    match (pos.screen == screen_data_end, pos.data == max_data) {
        // nowhere to go, nothing to change
        (true, true) => {
            None
        }
        // move data point
        (true, false) => {
            if pos.data + usize::from(step) >= max_data {
                pos.data += usize::from(step);
            } else {
                pos.data = max_data;
            }
            Some(pos)
        }
        // move screen point
        (false, true) => {
            if pos.screen + step <= screen_data_end {
                pos.screen += step;
            } else {
                pos.screen = screen_data_end;
            }
            Some(pos)
        }
        (false, false) => {
            if pos.screen + step <= inner.end {
                pos.screen += step;
                Some(pos)
            } else if pos.screen == inner.end {
                if pos.data + usize::from(step) <= max_data {
                    pos.data += usize::from(step);
                    Some(pos)
                } else {
                    let diff = 
                        u16::try_from(max_data.saturating_sub(pos.data))
                            .unwrap_or(u16::MIN);
                    step = step.saturating_sub(diff);
                    pos.data = max_data;
                    move_forward(inner, outer, &pos, dlength, step).or(Some(pos))
                }
            } else {
                let diff = inner.end.saturating_sub(pos.screen);
                step = step.saturating_sub(diff);
                pos.screen = inner.end;
                move_forward(inner, outer, &pos, dlength, step).or(Some(pos))
            }
        }
    }
}

// returns the start and end of displayable text
pub fn data_range(  
                    rng: &ScreenRange, 
                    pos: &PosCol, 
                    len: usize
                    ) -> DataRange 
{
    if len < rng.len() {
        DataRange {start: 0, end: len}
    } else {
        DataRange {
            start:  pos.data, 
            end:    std::cmp::min(pos.data + rng.len(), len),
        }
    }
}

pub fn move_left(   
                    dscr:   &DataScreen, 
                    pos:    &Pos, 
                    step:   u16 
                    ) -> Option<Pos> 
{
    let xinner  = dscr.inner.x();
    let xouter  = dscr.outer.x();
    let xcol    = pos.x();
    let ycol    = pos.y();

    move_backward(&xinner, &xouter, &xcol, step)
        .map(|x| x.join_with_y(ycol))
}

pub fn move_up( 
                dscr:   &DataScreen, 
                pos:    &Pos, 
                data:   &Vec<usize>,
                step:   u16 
                ) -> Option<Pos> 
{
    let yinner  = dscr.inner.y();
    let youter  = dscr.outer.y();
    let ycol    = pos.y();
    let xcol    = pos.x();

    move_backward(&yinner, &youter, &ycol, step)
        .map(|y| y.join_with_x(xcol))
        .map(|p| move_into_x(dscr, &p, data))
}

pub fn move_right(  
                    dscr:   &DataScreen, 
                    pos:    &Pos, 
                    data:   &Vec<usize>,
                    step:   u16 
                    ) -> Option<Pos> 
{
    let idx = dscr.outer.y().idx(&pos.y());
    let xlen    = if idx >= data.len() {0} else {data[idx]};
    let xinner  = dscr.inner.x();
    let xouter  = dscr.outer.x();
    let xcol    = pos.x();
    let ycol    = pos.y();

    move_forward(&xinner, &xouter, &xcol, xlen, step)
        .map(|x| x.join_with_y(ycol))
}

pub fn move_down(   
                    dscr:   &DataScreen, 
                    pos:    &Pos, 
                    data:   &Vec<usize>,
                    step:   u16 
                    ) -> Option<Pos> 
{
    let ylen    = data.len();
    let yinner  = dscr.inner.y();
    let youter  = dscr.outer.y();
    let ycol    = pos.y();
    let xcol    = pos.x();

    move_forward(&yinner, &youter, &ycol, ylen, step)
        .map(|y| y.join_with_x(xcol))
        .map(|p| move_into_x(dscr, &p, data))
}

pub fn move_into_x( 
                    dscr:   &DataScreen, 
                    pos:    &Pos,
                    data:   &Vec<usize>,
                    ) -> Pos 
{
    let xinner  = dscr.inner.x();
    let xouter  = dscr.outer.x();
    let xcol    = pos.x();
    let idx     = std::cmp::min(dscr.outer.y().idx(&pos.y()), data.len() - 1);
    let xlen    = data[idx];

    move_into(&xinner, &xouter, &xcol, xlen)
        .join_with_y(pos.y())
}

pub fn move_into_y( 
                    dscr:   &DataScreen, 
                    pos:    &Pos,
                    data:   &Vec<usize>,
                    ) -> Pos 
{
    let yinner  = dscr.inner.y();
    let youter  = dscr.outer.y();
    let ycol    = pos.y();
    let ylen    = data.len();

    move_into(&yinner, &youter, &ycol, ylen)
        .join_with_x(pos.x())
}

pub fn get_ranges(  
                    dscr:   &DataScreen, 
                    pos:    &Pos, 
                    data:   &Vec<usize>
                    ) -> Vec<(u16, usize, DataRange)> 
{
    let mut vec: Vec<(u16, usize, DataRange)> = vec![];
    let drng    = data_range(&dscr.outer.y(), &pos.y(), data.len());
    let xouter  = dscr.outer.x();
    let xcol    = pos.x();
    let ystart  = dscr.outer.y().start;

    for (e, i) in (drng.start..drng.end).into_iter().enumerate() {
        let rng = data_range(&xouter, &xcol, data[i]);
        let scr_idx = ystart + (e as u16);
        vec.push((scr_idx, i, rng));
    }
    vec
}
