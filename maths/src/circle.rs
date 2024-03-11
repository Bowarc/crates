use crate::Point;


#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
    pub fn center(&self) -> Point {
        self.center
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
