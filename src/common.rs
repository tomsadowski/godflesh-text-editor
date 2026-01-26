// common

#[derive(Clone)]
pub struct DataRange {
    pub a: usize, 
    pub b: usize
}

#[derive(Clone, Debug)]
pub struct ScreenRange {
    pub a: u16, 
    pub b: u16
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
        DataRange {a: 0, b: self.len()}
    }
    // index of cursor within its range
    pub fn idx(&self, col: &PosCol) -> usize {
        match col.cursor > self.a {
            true => 
                col.head + usize::from(col.cursor - self.a),
            false => 
                col.head,
        }

    }
    pub fn len(&self) -> usize {
        usize::from(self.b - self.a)
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
    pub fn ycut(&self, step: u16) -> Screen {
        match step < self.h {
            true => 
                Screen {
                    x: self.x, y: self.y,
                    w: self.w, h: self.h - step,
                },
            false => self.clone()
        }
    }
    pub fn xcut(&self, step: u16) -> Screen {
        match step < self.w {
            true => 
                Screen {
                    x: self.x, y: self.y,
                    w: self.w - step, h: self.h,
                },
            false => self.clone()
        }
    }
    pub fn yplus(&self, step: u16) -> Screen {
        match usize::from(step) * 2 < self.y().len() {
            true => 
                Screen {
                    x: self.x, y: self.y + step,
                    w: self.w, h: self.h - step,
                },
            false => self.clone()
        }
    }
    pub fn xplus(&self, step: u16) -> Screen {
        match usize::from(step) * 2 < self.x().len() {
            true => 
                Screen {
                    x: self.x + step, y: self.y,
                    w: self.w - step, h: self.h,
                },
            false => self.clone()
        }
    }
    pub fn x(&self) -> ScreenRange {
        ScreenRange {a: self.x, b: self.x + self.w}
    }
    pub fn y(&self) -> ScreenRange {
        ScreenRange {a: self.y, b: self.y + self.h}
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
    pub fn x(&self) -> PosCol {
        PosCol {cursor: self.x, head: self.i}
    }
    pub fn y(&self) -> PosCol {
        PosCol {cursor: self.y, head: self.j}
    }
}

#[derive(Clone, Debug)]
pub struct PosCol {
    pub cursor: u16, 
    pub head: usize
}
impl PosCol {
    pub fn join_with_x(self, x: PosCol) -> Pos {
        Pos {
            x: x.cursor, y: self.cursor,
            i: x.head, j: self.head,
        }
    }
    pub fn join_with_y(self, y: PosCol) -> Pos {
        Pos {
            x: self.cursor, y: y.cursor,
            i: self.head, j: y.head,
        }
    }
}

#[derive(Debug)]
pub enum Bound {
    Data(ScreenRange),
    Screen(usize),
    Space(ScreenRange, usize)
}
impl Bound {
    pub fn new(scr: ScreenRange, spc: u16, len: usize) -> Bound {
        if scr.len() >= len {
            Bound::Data(ScreenRange::from_length(scr.a, len))
        } else {
            if spc == 0 || usize::from(spc) * 2 >= scr.len() {
                Bound::Screen(len) 
            } else {
                let head_a      = scr.a + spc;
                let head_b      = scr.b - spc - 1;
                let head_points = ScreenRange::new(head_a, head_b);
                Bound::Space(head_points, len)
            }
        }
    }
    pub fn increment(self) -> Bound {
        match self {
            Bound::Data(rng) => {
                Bound::Data(ScreenRange {a: rng.a, b: rng.b + 1})
            }
            Bound::Space(rng, len) => {
                Bound::Space(rng, len + 1)
            }
            Bound::Screen(len) => {
                Bound::Screen(len + 1)
            }
        }
    }
    pub fn move_into(&self, rng: &ScreenRange, col: &PosCol) -> PosCol {
        let mut c = col.clone();
        match self {
            Bound::Data(rng) => {
                c.head = 0;
                let (a, b) = (rng.a, rng.b);
                match (c.cursor < a, c.cursor >= b) {
                    // cursor is less than a
                    (true, false) => {
                        c.cursor = a; c
                    }
                    // cursor is greater than or equal to b
                    (false, true) => {
                        c.cursor = b; c
                    }
                    _ => {c}
                } 
            }
            Bound::Space(rng, _) => {
                let (a, b) = (rng.a, rng.b);
                match (c.cursor < a, c.cursor >= b) {
                    // cursor is less than a
                    (true, false) => {
                        c.cursor = a; c
                    }
                    // cursor is greater than or equal to b
                    (false, true) => {
                        c.cursor = b; c
                    }
                    _ => {c}
                } 
            }
            Bound::Screen(_) => {
                let (a, b) = (rng.a, rng.b);
                match (c.cursor < a, c.cursor >= b) {
                    // cursor is less than a
                    (true, false) => {
                        c.cursor = a; c
                    }
                    // cursor is greater than or equal to b
                    (false, true) => {
                        c.cursor = b; c
                    }
                    _ => {c}
                } 
            }
        }
    }
    pub fn move_backward(&self, scr: &ScreenRange, col: &PosCol, step: u16) 
        -> Option<PosCol> 
    {
        let mut step = step;
        let mut c = col.clone();
        match self {
            Bound::Data(rng) => {
                if c.cursor == rng.a {
                    None
                } else if rng.a + step <= c.cursor {
                    c.cursor -= step;
                    Some(c)
                } else {
                    c.cursor = rng.a;
                    Some(c)
                }
            }
            Bound::Screen(_) => {
                match (c.cursor == scr.a, c.head == usize::MIN) {
                    (true, true) => {
                        None
                    }
                    (false, true) => {
                        if scr.a + step <= c.cursor {
                            c.cursor -= step;
                            Some(c)
                        } else {
                            c.cursor = scr.a;
                            Some(c)
                        }
                    }
                    (true, false) => {
                        if usize::from(step) < c.head  {
                            c.head -= usize::from(step);
                            Some(c)
                        } else {
                            c.head = usize::MIN;
                            Some(c)
                        }
                    }
                    (false, false) => {
                        if scr.a + step <= c.cursor {
                            c.cursor -= step;
                            Some(c)
                        } else {
                            step -= c.cursor - scr.a;
                            c.cursor = scr.a;
                            self.move_backward(scr, &c, step).or(Some(c))
                        }
                    }
                }
            }
            Bound::Space(rng, _) => {
                match (c.cursor == rng.a, c.head == usize::MIN) {
                    (_, true) => {
                        if c.cursor == scr.a {
                            None
                        } else if scr.a + step <= c.cursor {
                            c.cursor -= step;
                            Some(c)
                        } else {
                            c.cursor = scr.a;
                            Some(c)
                        }
                    }
                    (true, false) => {
                        if usize::from(step) < c.head  {
                            c.head -= usize::from(step);
                            Some(c)
                        } else {
                            step -= u16::try_from(c.head)
                                .unwrap_or(u16::MIN);
                            c.head = usize::MIN;
                            self.move_backward(scr, &c, step).or(Some(c))
                        }
                    }
                    (false, false) => {
                        if rng.a + step <= c.cursor {
                            c.cursor -= step;
                            Some(c)
                        } else {
                            step -= c.cursor - rng.a;
                            c.cursor = rng.a;
                            self.move_backward(scr, &c, step).or(Some(c))
                        }
                    }
                }
            }
        }
    }
    pub fn move_forward(&self, scr: &ScreenRange, col: &PosCol, step: u16) 
        -> Option<PosCol> 
    {
        let mut step = step;
        let mut c = col.clone();
        match self {
            Bound::Data(rng) => {
                if c.cursor == rng.b {
                    None
                } else if c.cursor + step < rng.b {
                    c.cursor += step;
                    Some(c)
                } else {
                    c.cursor = rng.b;
                    Some(c)
                }
            }
            Bound::Screen(_) => {
                match ( c.cursor == scr.b, 
                        c.head == self.max_head(scr)) 
                {
                    (true, true) => {
                        None
                    }
                    (false, false) => {
                        if c.cursor + step <= scr.b {
                            c.cursor += step;
                            Some(c)
                        } else {
                            step -= scr.b - c.cursor;
                            c.cursor = scr.b;
                            self.move_forward(scr, &c, step).or(Some(c))
                        }
                    }
                    (false, true) => {
                        if c.cursor + step <= scr.b {
                            c.cursor += step;
                            Some(c)
                        } else {
                            c.cursor = scr.b;
                            Some(c)
                        }
                    }
                    (true, false) => {
                        if c.head + usize::from(step) < 
                            self.max_head(scr)
                        {
                            c.head += usize::from(step);
                            Some(c)
                        } else {
                            c.head = self.max_head(scr);
                            Some(c)
                        }
                    }
                }
            }
            Bound::Space(rng, _) => {
                match ( c.cursor == rng.b, 
                        c.head == self.max_head(scr)) 
                {
                    (_, true) => {
                        if c.cursor == scr.b {
                            None
                        } else if c.cursor + step <= scr.b {
                            c.cursor += step;
                            Some(c)
                        } else {
                            c.cursor = scr.b;
                            Some(c)
                        }
                    }
                    (true, false) => {
                        if c.head + usize::from(step) < 
                            self.max_head(scr) 
                        {
                            c.head += usize::from(step);
                            Some(c)
                        } else {
                            match c.head < self.max_head(scr) {
                                true => {
                                    step += 
                                        u16::try_from(
                                            self.max_head(scr) - c.head)
                                        .unwrap_or(u16::MIN);
                                }
                                false => {}
                            };
                            c.head = self.max_head(scr);
                            self.move_forward(&scr, &c, step).or(Some(c))
                        }
                    }
                    (false, false) => {
                        if c.cursor + step <= rng.b {
                            c.cursor += step;
                            Some(c)
                        } else {
                            step += rng.b - c.cursor;
                            c.cursor = rng.b;
                            self.move_forward(scr, &c, step).or(Some(c))
                        }
                    }
                }
            }
        }
    }
    pub fn max_head(&self, rng: &ScreenRange) -> usize {
        match self {
            Bound::Screen(l) | Bound::Space(_, l) => 
                l - rng.len(),
            _ => 0,
        }
    }
    // returns the start and end of displayable text
    pub fn drng(&self, scr: &ScreenRange, col: &PosCol) -> DataRange {
        match self {
            Bound::Data(range) => {
                range.to_data_range()
            }
            Bound::Screen(len) |
            Bound::Space(_, len) => {
                DataRange {
                    a: col.head, 
                    b: std::cmp::min(col.head + scr.len(), *len),
                }
            }
        }
    }
}
pub struct Page {
    pub scr: Screen,
    pub x:   Vec<Bound>,
    pub y:   Bound,
}
impl Page {
    pub fn new(scr: &Screen, txt: &Vec<String>, hspc: u16, vspc: u16) -> Page {
        Self {
            x: txt.iter()
                .map(|txt| Bound::new(scr.x(), hspc, txt.len()))
                .collect(),
            y:   Bound::new(scr.y(), vspc, txt.len()), 
            scr: scr.clone(),
        }
    }
    pub fn move_left(&self, pos: &Pos, step: u16) -> Option<Pos> {
        self.x(&pos)
            .move_backward(&self.scr.x(), &pos.x(), step)
            .map(|x| x.join_with_y(pos.y()))
    }
    pub fn move_right(&self, pos: &Pos, step: u16) -> Option<Pos> {
        self.x(&pos)
            .move_forward(&self.scr.x(), &pos.x(), step)
            .map(|x| x.join_with_y(pos.y()))
    }
    pub fn move_up(&self, pos: &Pos, step: u16) -> Option<Pos> {
        self.y
            .move_backward(&self.scr.y(), &pos.y(), step)
            .map(|y| y.join_with_x(pos.x()))
            .map(|p| self.move_into_x(&p))
    }
    pub fn move_down(&self, pos: &Pos, step: u16) -> Option<Pos> {
        self.y
            .move_forward(&self.scr.y(), &pos.y(), step)
            .map(|y| y.join_with_x(pos.x()))
            .map(|p| self.move_into_x(&p))
    }
    pub fn move_into_y(&self, pos: &Pos) -> Pos {
        self.y
            .move_into(&self.scr.y(), &pos.y())
            .join_with_x(pos.x())
    }
    pub fn move_into_x(&self, pos: &Pos) -> Pos {
        self.x(&pos)
            .move_into(&self.scr.x(), &pos.x())
            .join_with_y(pos.y())
    }
    pub fn get_ranges(&self, pos: &Pos) -> Vec<(u16, usize, DataRange)> {
        let mut vec: Vec<(u16, usize, DataRange)> = vec![];
        let drng = self.y.drng(&self.scr.y(), &pos.y());
        for (e, i) in (drng.a..drng.b).into_iter().enumerate() {
            let rng = self.x[i].drng(&self.scr.x(), &pos.x());
            vec.push((self.scr.y().a + (e as u16), i, rng));
        }
        vec
    }
    pub fn x(&self, pos: &Pos) -> &Bound {
        let idx = self.scr.y().idx(&pos.y());
        let len = self.x.len();
        match idx >= len {
            true => 
                &Bound::Data(ScreenRange {a: 0, b: 0}),
            false => 
                &self.x[idx]
        }
    }
}
