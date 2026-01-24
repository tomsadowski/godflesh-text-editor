// common

#[derive(Clone)]
pub struct DataRange {
    pub a: usize, 
    pub b: usize
}
#[derive(Clone)]
pub struct ScreenRange {
    pub a: u16, 
    pub b: u16
}
#[derive(Clone)]
pub struct Screen {
    pub x: u16, 
    pub y: u16,
    pub w: u16, 
    pub h: u16
}
#[derive(Clone)]
pub struct Pos {
    pub x: u16, 
    pub y: u16,
    pub i: usize, 
    pub j: usize
}
#[derive(Clone)]
pub struct PosCol {
    pub cursor: u16, 
    pub scroll: usize
}
pub enum Bound {
    Data(ScreenRange),
    Screen(usize),
    Space(ScreenRange, usize)
}
pub struct Page {
    pub scr:  Screen,
    pub xbnd: Vec<Bound>,
    pub ybnd: Bound,
}
pub trait Subject {
    fn get_x_bnd(&self, pos: &Pos) -> &Bound;
    fn get_y_bnd(&self, pos: &Pos) -> &Bound;
    fn get_x_rng(&self) -> ScreenRange;
    fn get_y_rng(&self) -> ScreenRange;
}

impl PosCol {
    pub fn join_with_x(self, x: PosCol) -> Pos {
        Pos {
            x: x.cursor, y: self.cursor,
            i: x.scroll, j: self.scroll,
        }
    }
    pub fn join_with_y(self, y: PosCol) -> Pos {
        Pos {
            x: self.cursor, y: y.cursor,
            i: self.scroll, j: y.scroll,
        }
    }
    pub fn move_into(&mut self, rng: &ScreenRange, bnd: &Bound) -> bool {
        let (a, b) = 
            match bnd {
                Bound::Data(rng) | Bound::Space(rng, _) => 
                    (rng.a, rng.b),
                Bound::Screen(_) => 
                    (rng.a, rng.b), 
            };
        match (self.cursor >= a, self.cursor <= b) {
            (true, true) => {
                false
            }
            (true, false) => {
                self.cursor = a;
                true
            }
            (_, _) => {
                self.cursor = b;
                true
            }
        } 
    }
    pub fn move_backward(   &mut self, 
                            rng: &ScreenRange, 
                            bnd: &Bound, 
                            step: u16   ) -> bool 
    {
        let mut step = step;
        match bnd {
            Bound::Data(rng) => {
                if self.cursor == rng.a {
                    false
                } else if rng.a + step <= self.cursor {
                    self.cursor -= step;
                    true
                } else {
                    self.cursor = rng.a;
                    true
                }
            }
            Bound::Screen(_) => {
                match (self.cursor == rng.a, self.scroll == usize::MIN) {
                    (true, true) => {
                        false
                    }
                    (false, true) => {
                        if rng.a + step <= self.cursor {
                            self.cursor -= step;
                            true
                        } else {
                            self.cursor = rng.a;
                            true
                        }
                    }
                    (true, false) => {
                        if usize::from(step) < self.scroll  {
                            self.scroll -= usize::from(step);
                            true
                        } else {
                            self.scroll = usize::MIN;
                            true
                        }
                    }
                    (false, false) => {
                        if rng.a + step <= self.cursor {
                            self.cursor -= step;
                            true
                        } else {
                            step -= self.cursor - rng.a;
                            self.cursor = rng.a;
                            self.move_backward(rng, bnd, step);
                            true
                        }
                    }
                }
            }
            Bound::Space(scroll, _) => {
                match (self.cursor == scroll.a, self.scroll == usize::MIN) {
                    (_, true) => {
                        if rng.a + step <= self.cursor {
                            self.cursor -= step;
                            true
                        } else {
                            self.cursor = rng.a;
                            true
                        }
                    }
                    (true, false) => {
                        if usize::from(step) < self.scroll  {
                            self.scroll -= usize::from(step);
                            true
                        } else {
                            step -= u16::try_from(self.scroll)
                                .unwrap_or(u16::MIN);
                            self.scroll = usize::MIN;
                            self.move_backward(rng, bnd, step);
                            true
                        }
                    }
                    (false, false) => {
                        if scroll.a + step <= self.cursor {
                            self.cursor -= step;
                            true
                        } else {
                            step -= self.cursor - scroll.a;
                            self.cursor = scroll.a;
                            self.move_backward(rng, bnd, step);
                            true
                        }
                    }
                }
            }
        }
    }
    pub fn move_forward(    &mut self, 
                            rng: &ScreenRange, 
                            bnd: &Bound, 
                            step: u16   )  -> bool 
    {
        let mut step = step;
        match bnd {
            Bound::Data(rng) => {
                if self.cursor == rng.b {
                    false
                } else if rng.b + step <= self.cursor {
                    self.cursor -= step;
                    true
                } else {
                    self.cursor = rng.b;
                    true
                }
            }
            Bound::Screen(_) => {
                match ( self.cursor == rng.b, 
                        self.scroll == bnd.get_max_scroll(rng)) 
                {
                    (true, true) => {
                        false
                    }
                    (false, true) => {
                        if self.cursor + step <= rng.b {
                            self.cursor += step;
                            true
                        } else {
                            self.cursor = rng.b;
                            true
                        }
                    }
                    (true, false) => {
                        if self.scroll + usize::from(step) < 
                            bnd.get_max_scroll(rng)  
                        {
                            self.scroll += usize::from(step);
                            true
                        } else {
                            self.scroll = bnd.get_max_scroll(rng);
                            true
                        }
                    }
                    (false, false) => {
                        if self.cursor + step <= rng.b {
                            self.cursor += step;
                            true
                        } else {
                            step -= rng.b - self.cursor;
                            self.cursor = rng.b;
                            self.move_forward(rng, bnd, step);
                            true
                        }
                    }
                }
            }
            Bound::Space(scroll, _) => {
                match ( self.cursor <= scroll.b, 
                        self.scroll == bnd.get_max_scroll(rng)) 
                {
                    (_, true) => {
                        if self.cursor == rng.b {
                            false
                        } else if self.cursor + step <= rng.b {
                            self.cursor += step;
                            true
                        } else {
                            self.cursor = rng.b;
                            true
                        }
                    }
                    (true, false) => {
                        if self.scroll + usize::from(step) < 
                            bnd.get_max_scroll(rng) 
                        {
                            self.scroll += usize::from(step);
                            true
                        } else {
                            step -= u16::try_from(
                                    bnd.get_max_scroll(rng) - self.scroll)
                                .unwrap_or(u16::MIN);
                            self.scroll = bnd.get_max_scroll(rng);
                            self.move_forward(&rng, bnd, step);
                            true
                        }
                    }
                    (false, false) => {
                        if self.cursor + step <= scroll.b {
                            self.cursor += step;
                            true
                        } else {
                            step += scroll.b - self.cursor;
                            self.cursor = scroll.b;
                            self.move_forward(rng, bnd, step);
                            true
                        }
                    }
                }
            }
        }
    }
}
impl Screen {
    pub fn xrng(&self) -> ScreenRange {
        let x = self.x;
        let w = self.w;
        ScreenRange {a: x, b: x + w}
    }
    pub fn yrng(&self) -> ScreenRange {
        let y = self.y;
        let h = self.h;
        ScreenRange {a: y, b: y + h}
    }
}
impl ScreenRange {
    pub fn from_length(a: u16, len: usize) -> ScreenRange {
        let len = u16::try_from(len).unwrap_or(u16::MIN);
        ScreenRange {a: a, b: a + len}
    }
    // if for some reason a > b, just swap them
    pub fn new(a: u16, b: u16) -> ScreenRange {
        match a > b {
            true =>  ScreenRange {a: b, b: a},
            false => ScreenRange {a: a, b: b},
        }
    }
    pub fn to_data_range(&self) -> DataRange {
        DataRange {a: usize::from(self.a), b: usize::from(self.b)}
    }
    // index of cursor within its range
    pub fn get_idx(&self, col: &PosCol) -> usize {
        col.scroll + usize::from(col.cursor - self.a)
    }
    pub fn length(&self) -> usize {
        usize::from(self.b - self.a)
    }
}
impl Bound {
    pub fn new(rng: ScreenRange, spc: u16, len: usize) -> Bound {
        if rng.length() >= len {
            Bound::Data(ScreenRange::from_length(rng.a, len))
        } else {
            if usize::from(spc) * 2 >= rng.length() {
                Bound::Screen(len) 
            } else {
                let scroll_a      = rng.a + spc;
                let scroll_b      = rng.b - spc - 1;
                let scroll_points = ScreenRange::new(scroll_a, scroll_b);
                Bound::Space(scroll_points, len)
            }
        }
    }
    pub fn get_max_scroll(&self, rng: &ScreenRange) -> usize {
        match self {
            Bound::Screen(l) | Bound::Space(_, l) => 
                l - rng.length(),
            _ => 0,
        }
    }
    // returns the start and end of displayable text
    pub fn get_data_range(&self, scr: &ScreenRange, col: &PosCol) -> DataRange {
        match self {
            Bound::Data(range) => 
                range.to_data_range(),
            _ => 
                DataRange {a: col.scroll, b: col.scroll + scr.length()},
            
        }
    }
}
impl Pos {
    pub fn xcol(&self) -> PosCol {
        let cursor = self.x;
        let scroll = self.i;
        PosCol {cursor, scroll}
    }
    pub fn get_y(&self) -> PosCol {
        let cursor = self.y;
        let scroll = self.j;
        PosCol {cursor, scroll}
    }
    pub fn move_left<T: Subject>(&mut self, subject: &T, step: u16) -> bool {
        let mut xcol = self.xcol();
        let xbnd = subject.get_x_bnd(&self);
        let xrng = subject.get_x_rng();
        match xcol.move_backward(&xrng, xbnd, step) {
            true => {
                self.x = xcol.cursor;
                self.i = xcol.scroll;
                true
            }
            false => {
                false
            }
        }
    }
    pub fn move_right<T: Subject>(&mut self, subject: &T, step: u16) -> bool {
        let mut xcol = self.xcol();
        let xbnd = subject.get_x_bnd(&self);
        let xrng = subject.get_x_rng();
        match xcol.move_forward(&xrng, xbnd, step) {
            true => {
                self.x = xcol.cursor;
                self.i = xcol.scroll;
                true
            }
            false => {
                false
            }
        }
    }
    pub fn move_up<T: Subject>(&mut self, subject: &T, step: u16) -> bool {
        let mut ycol = self.get_y();
        let ybnd = subject.get_y_bnd(&self);
        let yrng = subject.get_y_rng();
        match ycol.move_backward(&yrng, ybnd, step) {
            true => {
                self.y = ycol.cursor;
                self.j = ycol.scroll;
                true
            }
            false => {
                false
            }
        }
    }
    pub fn move_down<T: Subject>(&mut self, subject: &T, step: u16) -> bool {
        let mut ycol = self.get_y();
        let ybnd = subject.get_y_bnd(&self);
        let yrng = subject.get_y_rng();
        match ycol.move_forward(&yrng, ybnd, step) {
            true => {
                self.y = ycol.cursor;
                self.j = ycol.scroll;
                true
            }
            false => {
                false
            }
        }
    }
}
impl Page {
    pub fn new(scr: &Screen, txt: &Vec<String>, hspc: u16, vspc: u16) -> Page {
        let xbnd = txt.iter()
            .map(|txt| Bound::new(scr.xrng(), hspc, txt.len()));
        let ybnd = Bound::new(scr.yrng(), vspc, txt.len());
        Self {
            xbnd: xbnd.collect(), 
            ybnd: ybnd, 
            scr:  scr.clone(),
        }
    }
    pub fn get_ranges(&self, pos: &Pos) 
        -> Vec<(u16, usize, DataRange)> 
    {
        let xrng = self.scr.xrng();
        let yrng = self.scr.yrng();
        let xcol = pos.xcol();
        let drng = self.get_y_data_range(pos);
        let mut vec: Vec<(u16, usize, DataRange)> = vec![];
        for (e, i) in (drng.a..drng.b).into_iter().enumerate() {
            let rng = self.xbnd[i].get_data_range(&xrng, &xcol);
            vec.push((yrng.a + (e as u16), i, rng));
        }
        vec
    }
    pub fn get_y_data_range(&self, pos: &Pos) -> DataRange {
        self.ybnd.get_data_range(&self.scr.yrng(), &pos.get_y())
    }
}
impl Subject for Page {
    fn get_x_rng(&self) -> ScreenRange {
        self.scr.xrng()
    }
    fn get_y_rng(&self) -> ScreenRange {
        self.scr.yrng()
    }
    fn get_x_bnd(&self, pos: &Pos) -> &Bound {
        &self.xbnd[self.scr.yrng().get_idx(&pos.get_y())]
    }
    fn get_y_bnd(&self, _: &Pos) -> &Bound {
        &self.ybnd
    }
}
