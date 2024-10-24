#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(from = "((f64, f64), f64)")
)]
pub struct Circle {
    pub center: super::Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: super::Point, radius: f64) -> Self {
        Self { center, radius }
    }
    pub fn center(&self) -> super::Point {
        self.center
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<((f64, f64), f64)> for Circle {
    fn from(value: ((f64, f64), f64)) -> Self {
        Circle {
            center: super::Point::from(value.0),
            radius: value.1,
        }
    }
}
