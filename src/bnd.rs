// bnd

use crate::{
    scr::{Screen, ScreenRange, DataRange, Pos, PosCol},
};

#[derive(Debug)]
pub enum Bound {
    Data(ScreenRange),
    Screen(usize),
    Space(ScreenRange, usize)
}
impl Bound {
    pub fn new(srng: ScreenRange, spc: u16, len: usize) -> Bound {
        if srng.len() >= len {
            Bound::Data(ScreenRange::from_length(srng.start, len))
        } else {
            if spc == 0 || usize::from(spc) * 2 >= srng.len() {
                Bound::Screen(len) 
            } else {
                let start    = srng.start + spc;
                let end      = srng.end - spc - 1;
                Bound::Space(ScreenRange::new(start, end), len)
            }
        }
    }
    pub fn increment(self) -> Bound {
        match self {
            Bound::Data(rng) => {
                Bound::Data(ScreenRange::new(rng.start, rng.end + 1))
            }
            Bound::Space(rng, len) => {
                Bound::Space(rng, len + 1)
            }
            Bound::Screen(len) => {
                Bound::Screen(len + 1)
            }
        }
    }
    pub fn move_into(&self, srng: &ScreenRange, pos: &PosCol) -> PosCol {
        let mut pos = pos.clone();
        let (start, end) = match self {
            Bound::Data(rng) => {
                pos.data = 0;
                (rng.start, rng.end)
            }
            Bound::Space(rng, _) => {
                (rng.start, rng.end)
            }
            Bound::Screen(_) => {
                (srng.start, srng.end)
            }
        };
        match (pos.screen < start, pos.screen >= end) {
            // cursor is less than a
            (true, false) => {
                pos.screen = start; pos
            }
            // cursor is greater than or equal to b
            (false, true) => {
                pos.screen = end; pos
            }
            _ => {pos}
        }
    }
    pub fn move_backward(&self, srng: &ScreenRange, pos: &PosCol, step: u16) 
        -> Option<PosCol> 
    {
        let mut step = step;
        let mut pos = pos.clone();
        match self {
            Bound::Data(rng) => {
                if pos.screen == rng.start {
                    None
                } else if rng.start + step <= pos.screen {
                    pos.screen -= step;
                    Some(pos)
                } else {
                    pos.screen = rng.start;
                    Some(pos)
                }
            }
            Bound::Screen(_) => {
                match (pos.screen == srng.start, pos.data == usize::MIN) {
                    (true, true) => {
                        None
                    }
                    (false, true) => {
                        if srng.start + step <= pos.screen {
                            pos.screen -= step;
                            Some(pos)
                        } else {
                            pos.screen = srng.start;
                            Some(pos)
                        }
                    }
                    (true, false) => {
                        if usize::from(step) < pos.data  {
                            pos.data -= usize::from(step);
                            Some(pos)
                        } else {
                            pos.data = usize::MIN;
                            Some(pos)
                        }
                    }
                    (false, false) => {
                        if srng.start + step <= pos.screen {
                            pos.screen -= step;
                            Some(pos)
                        } else {
                            step -= pos.screen - srng.start;
                            pos.screen = srng.start;
                            self.move_backward(srng, &pos, step).or(Some(pos))
                        }
                    }
                }
            }
            Bound::Space(rng, _) => {
                match (pos.screen == rng.start, pos.data == usize::MIN) {
                    (_, true) => {
                        if pos.screen == srng.start {
                            None
                        } else if srng.start + step <= pos.screen {
                            pos.screen -= step;
                            Some(pos)
                        } else {
                            pos.screen = srng.start;
                            Some(pos)
                        }
                    }
                    (true, false) => {
                        if usize::from(step) < pos.data  {
                            pos.data -= usize::from(step);
                            Some(pos)
                        } else {
                            step -= u16::try_from(pos.data)
                                .unwrap_or(u16::MIN);
                            pos.data = usize::MIN;
                            self.move_backward(srng, &pos, step).or(Some(pos))
                        }
                    }
                    (false, false) => {
                        if rng.start + step <= pos.screen {
                            pos.screen -= step;
                            Some(pos)
                        } else {
                            step -= pos.screen - rng.start;
                            pos.screen = rng.start;
                            self.move_backward(srng, &pos, step).or(Some(pos))
                        }
                    }
                }
            }
        }
    }
    pub fn move_forward(&self, srng: &ScreenRange, pos: &PosCol, step: u16) 
        -> Option<PosCol> 
    {
        let mut step = step;
        let mut pos = pos.clone();
        match self {
            Bound::Data(rng) => {
                if pos.screen == rng.end {
                    None
                } else if pos.screen + step < rng.end {
                    pos.screen += step;
                    Some(pos)
                } else {
                    pos.screen = rng.end;
                    Some(pos)
                }
            }
            Bound::Screen(_) => {
                match ( pos.screen == srng.end, 
                        pos.data == self.max_data(srng)) 
                {
                    (true, true) => {
                        None
                    }
                    (false, false) => {
                        if pos.screen + step <= srng.end {
                            pos.screen += step;
                            Some(pos)
                        } else {
                            step -= srng.end - pos.screen;
                            pos.screen = srng.end;
                            self.move_forward(srng, &pos, step).or(Some(pos))
                        }
                    }
                    (false, true) => {
                        if pos.screen + step <= srng.end {
                            pos.screen += step;
                            Some(pos)
                        } else {
                            pos.screen = srng.end;
                            Some(pos)
                        }
                    }
                    (true, false) => {
                        if pos.data + usize::from(step) < 
                            self.max_data(srng)
                        {
                            pos.data += usize::from(step);
                            Some(pos)
                        } else {
                            pos.data = self.max_data(srng);
                            Some(pos)
                        }
                    }
                }
            }
            Bound::Space(rng, _) => {
                match ( pos.screen == rng.end, 
                        pos.data == self.max_data(srng)) 
                {
                    (_, true) => {
                        if pos.screen == srng.end {
                            None
                        } else if pos.screen + step <= srng.end {
                            pos.screen += step;
                            Some(pos)
                        } else {
                            pos.screen = srng.end;
                            Some(pos)
                        }
                    }
                    (true, false) => {
                        if pos.data + usize::from(step) < 
                            self.max_data(srng) 
                        {
                            pos.data += usize::from(step);
                            Some(pos)
                        } else {
                            match pos.data < self.max_data(srng) {
                                true => {
                                    step += 
                                        u16::try_from(
                                            self.max_data(srng) - pos.data)
                                        .unwrap_or(u16::MIN);
                                }
                                false => {}
                            };
                            pos.data = self.max_data(srng);
                            self.move_forward(&srng, &pos, step).or(Some(pos))
                        }
                    }
                    (false, false) => {
                        if pos.screen + step <= rng.end {
                            pos.screen += step;
                            Some(pos)
                        } else {
                            step += rng.end - pos.screen;
                            pos.screen = rng.end;
                            self.move_forward(srng, &pos, step).or(Some(pos))
                        }
                    }
                }
            }
        }
    }
    pub fn max_data(&self, rng: &ScreenRange) -> usize {
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
                    start: col.data, 
                    end: std::cmp::min(col.data + scr.len(), *len),
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
        for (e, i) in (drng.start..drng.end).into_iter().enumerate() {
            let rng = self.x[i].drng(&self.scr.x(), &pos.x());
            vec.push((self.scr.y().start + (e as u16), i, rng));
        }
        vec
    }
    pub fn x(&self, pos: &Pos) -> &Bound {
        let idx = self.scr.y().idx(&pos.y());
        let len = self.x.len();
        match idx >= len {
            true => 
                &Bound::Data(ScreenRange {start: 0, end: 0}),
            false => 
                &self.x[idx]
        }
    }
}
