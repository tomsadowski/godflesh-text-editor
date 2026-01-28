use std::cmp::min;

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
    // index of cursor within its range
    pub fn idx(&self, col: &PosCol) -> usize {
        if col.screen > self.start {
            col.data + usize::from(col.screen.saturating_sub(self.start))
        } else {
            col.data
        }
    }
    pub fn len(&self) -> usize {
        usize::from(self.end.saturating_sub(self.start))
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
    pub fn crop_y(&self, step: u16) -> Screen {
        let screen = self.clone();
        screen.crop_north(step).crop_south(step)
    }
    pub fn crop_x(&self, step: u16) -> Screen {
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
        if usize::from(step) * 2 < screen.y_rng().len() {
            screen.y += step;
            screen.h -= step;
        }
        screen
    }
    pub fn crop_west(&self, step: u16) -> Screen {
        let mut screen = self.clone();
        if usize::from(step) * 2 < screen.x_rng().len() {
            screen.x += step;
            screen.w -= step;
        }
        screen
    }
    pub fn x_rng(&self) -> ScreenRange {
        ScreenRange {start: self.x, end: self.x + self.w}
    }
    pub fn y_rng(&self) -> ScreenRange {
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
            inner: outer.crop_x(x).crop_y(y),
            outer: outer,
        }
    }
}
#[derive(Clone, Debug)]
pub struct PosCol {
    pub screen: u16, 
    pub data: usize
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
    pub fn x_col(&self) -> PosCol {
        PosCol {screen: self.x, data: self.i}
    }
    pub fn y_col(&self) -> PosCol {
        PosCol {screen: self.y, data: self.j}
    }
    pub fn join(x: &PosCol, y: &PosCol) -> Pos {
        Pos {
            x: x.screen, 
            y: y.screen,
            i: x.data, 
            j: y.data,
        }
    }
}
pub fn u16_or_zero(u: usize) -> u16 {
    u16::try_from(u).unwrap_or(u16::MIN)
}
pub fn move_into(   inner: &ScreenRange, 
                    outer: &ScreenRange, 
                    pos: &PosCol,
                    len: usize  ) -> PosCol 
{
    let mut pos = pos.clone();
    let (start, end) = 
        if len < outer.len() {
            pos.data = 0;
            let len = u16_or_zero(len);
            (outer.start, outer.start + len)
        } else {
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
pub fn move_backward(   inner:  &ScreenRange, 
                        outer:  &ScreenRange, 
                        pos:    &PosCol, 
                        step:   u16 ) -> Option<PosCol> 
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
                    step -= u16_or_zero(pos.data);
                    pos.data = usize::MIN;
                    move_backward(inner, outer, &pos, step)
                        .or(Some(pos))
                }
            } else {
                step -= pos.screen - inner.start;
                pos.screen = inner.start;
                move_backward(inner, outer, &pos, step)
                    .or(Some(pos))
            }
        }
    }
}
pub fn move_forward(    inner:      &ScreenRange, 
                        outer:      &ScreenRange, 
                        pos:        &PosCol, 
                        dlength:    usize,
                        step:       u16 ) -> Option<PosCol> 
{
    let screen_data_end = u16_or_zero(min(
        usize::from(outer.start) + dlength, 
        usize::from(outer.end)));
    let mut step    = step;
    let mut pos     = pos.clone();
    let screen_len  = outer.len();
    let max_data    = dlength.saturating_sub(screen_len);
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
                    let diff = u16_or_zero(max_data.saturating_sub(pos.data));
                    step = step.saturating_sub(diff);
                    pos.data = max_data;
                    move_forward(inner, outer, &pos, dlength, step)
                        .or(Some(pos))
                }
            } else {
                let diff = inner.end.saturating_sub(pos.screen);
                step = step.saturating_sub(diff);
                pos.screen = inner.end;
                move_forward(inner, outer, &pos, dlength, step)
                    .or(Some(pos))
            }
        }
    }
}
// returns the start and end of displayable text
pub fn data_range(rng: &ScreenRange, pos: &PosCol, len: usize) 
    -> (usize, usize) 
{
    if len < rng.len() {
        (0, len)
    } else {
        (pos.data, min(pos.data + rng.len(), len))
    }
}
pub fn move_left(dscr: &DataScreen, pos: &Pos, step: u16) -> Option<Pos> {
    let x_inner = dscr.inner.x_rng();
    let x_outer = dscr.outer.x_rng();
    let x_col   = pos.x_col();
    let y_col   = pos.y_col();
    move_backward(&x_inner, &x_outer, &x_col, step)
        .map(|x_col| Pos::join(&x_col, &y_col))
}
pub fn move_right(  dscr:   &DataScreen, 
                    pos:    &Pos, 
                    data:   &Vec<usize>,
                    step:   u16 ) -> Option<Pos> 
{
    let x_inner = dscr.inner.x_rng();
    let x_outer = dscr.outer.x_rng();
    let x_col   = pos.x_col();
    let y_col   = pos.y_col();
    let idx     = dscr.outer.y_rng().idx(&y_col);
    let x_len   = if idx >= data.len() {0} else {data[idx]};
    move_forward(&x_inner, &x_outer, &x_col, x_len, step)
        .map(|x_col| Pos::join(&x_col, &y_col))
}
pub fn move_up( dscr:   &DataScreen, 
                pos:    &Pos, 
                data:   &Vec<usize>,
                step:   u16 ) -> Option<Pos> 
{
    let y_inner = dscr.inner.y_rng();
    let y_outer = dscr.outer.y_rng();
    let y_col   = pos.y_col();
    let x_col   = pos.x_col();
    move_backward(&y_inner, &y_outer, &y_col, step)
        .map(|y_col| Pos::join(&x_col, &y_col))
        .map(|pos| move_into_x(dscr, &pos, data))
}
pub fn move_down(   dscr:   &DataScreen, 
                    pos:    &Pos, 
                    data:   &Vec<usize>,
                    step:   u16 ) -> Option<Pos> 
{
    let y_len   = data.len();
    let y_inner = dscr.inner.y_rng();
    let y_outer = dscr.outer.y_rng();
    let x_col   = pos.x_col();
    let y_col   = pos.y_col();
    move_forward(&y_inner, &y_outer, &y_col, y_len, step)
        .map(|y_col| Pos::join(&x_col, &y_col))
        .map(|pos| move_into_x(dscr, &pos, data))
}
pub fn resize(dscr: &DataScreen, pos: &Pos, data: &Vec<usize>) -> Pos {
    let pos = move_into_x(dscr, pos, data);
    move_into_y(dscr, &pos, data)
}
pub fn move_into_x(dscr: &DataScreen, pos: &Pos, data: &Vec<usize>) -> Pos {
    let x_inner = dscr.inner.x_rng();
    let x_outer = dscr.outer.x_rng();
    let x_col   = pos.x_col();
    let y_col   = pos.y_col();
    let idx1    = dscr.outer.y_rng().idx(&y_col);
    let idx2    = data.len().saturating_sub(1);
    let idx     = min(idx1, idx2);
    let x_len   = data[idx];
    let x_col   = move_into(&x_inner, &x_outer, &x_col, x_len);
    Pos::join(&x_col, &y_col)
}
pub fn move_into_y(dscr: &DataScreen, pos: &Pos, data: &Vec<usize>) -> Pos {
    let y_inner = dscr.inner.y_rng();
    let y_outer = dscr.outer.y_rng();
    let y_col   = pos.y_col();
    let x_col   = pos.x_col();
    let y_len   = data.len();
    let y_col   = move_into(&y_inner, &y_outer, &y_col, y_len);
    Pos::join(&x_col, &y_col)
}
pub fn get_ranges(dscr: &DataScreen, pos: &Pos, data: &Vec<usize>) 
    -> Vec<(u16, usize, usize, usize)> 
{
    let mut vec: Vec<(u16, usize, usize, usize)> = vec![];
    let x_col   = pos.x_col();
    let y_col   = pos.y_col();
    let x_outer = dscr.outer.x_rng();
    let y_outer = dscr.outer.y_rng();
    let (start, end) = data_range(&y_outer, &y_col, data.len());
    for (e, i) in (start..end).into_iter().enumerate() {
        let (a, b) = data_range(&x_outer, &x_col, data[i]);
        let scr_idx = y_outer.start + (e as u16);
        vec.push((scr_idx, i, a, b));
    }
    vec
}
