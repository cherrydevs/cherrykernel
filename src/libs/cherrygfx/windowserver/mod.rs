// cherrygfx.windowserver v0.1.1

pub struct Window {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
    pub id: usize,
}

impl Window {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        let mut wind = Self { x, y, w, h, id: 0 };
        return wind;
    }
}