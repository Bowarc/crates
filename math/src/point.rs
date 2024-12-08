#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const ZERO: Self = Self { x: 0., y: 0. };
    pub const ONE: Self = Self { x: 1., y: 1. };
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn new_rotated(origin: impl Into<Point>, pt: impl Into<Point>, angle: f64) -> Self {
        let origin = origin.into();
        let pt = pt.into();

        if angle == 0. {
            return pt;
        }
        // Self {
        //     x: 0. + a.cos() * (x - 0.) - a.sin() * (y - 0.),
        //     y: 0. + a.sin() * (x - 0.) + a.cos() * (y - 0.),
        // };

        Point::new(
            origin.x + angle.cos() * (pt.x - origin.x) - angle.sin() * (pt.y - origin.y),
            origin.y + angle.sin() * (pt.x - origin.x) + angle.cos() * (pt.y - origin.y),
        )
    }
    pub fn from_angle(a: f64) -> Self {
        Self::new(a.cos(), a.sin())
    }
    // in randians
    pub fn as_angle(&self) -> f64 {
        two_points_angle(Point::ZERO, *self)
    }
    pub fn normalize(&mut self) {
        let d = ((self.x * self.x) + (self.y * self.y)).sqrt();
        if d != 0.0 {
            self.x /= d;
            self.y /= d;
        } else {
            warn!("Test warn this should never happend, if you see this message please contact one of the devs.\n(error: crate::Point::normalize created a number less than 0)")
        }
        // self.x /= d;
        // self.y /= d;
    }
    pub fn dot(&self, other: Self) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }
    // pub fn project_onto(&self, other: Self) -> Self {
    //     let other_len_sq_rcp = other.dot(-other).recip();

    //     if other_len_sq_rcp == 0. {
    //         panic!("You should not project onto a zero vector")
    //     }
    //     other * self.dot(other) * other_len_sq_rcp
    // }
    // pub fn reject_from(&mut self, other: Self) {
    //     // *self = self.project_onto(other)
    //     *self += other;
    //     self.normalize()
    // }
    pub fn round(&mut self) {
        *self = self.rounded()
    }
    pub fn rounded(&self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }

    pub fn floor(&mut self) {
        *self = self.floored()
    }
    pub fn floored(&self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }
}

pub fn normalize_p2(pt: Point) -> Point {
    let d = ((pt.x * pt.x) + (pt.y * pt.y)).sqrt();
    if d != 0.0 {
        Point::new(pt.x / d, pt.y / d)
    } else {
        pt
    }
}

pub fn two_points_angle(base: Point, target: Point) -> f64 {
    (target.y - base.y).atan2(target.x - base.x)
}

#[cfg(feature = "ggez")]
impl std::convert::From<Point> for ggez::mint::Point2<f64> {
    fn from(o: Point) -> Self {
        ggez::mint::Point2 { x: o.x, y: o.y }
    }
}
#[cfg(feature = "ggez")]
impl std::convert::From<Point> for ggez::mint::Vector2<f64> {
    fn from(o: Point) -> Self {
        ggez::mint::Vector2 { x: o.x, y: o.y }
    }
}
#[cfg(feature = "ggez")]
impl std::convert::From<Point> for ggez::mint::Point2<f32> {
    fn from(o: Point) -> Self {
        ggez::mint::Point2 {
            x: o.x as f32,
            y: o.y as f32,
        }
    }
}
#[cfg(feature = "ggez")]
impl std::convert::From<Point> for ggez::mint::Vector2<f32> {
    fn from(o: Point) -> Self {
        ggez::mint::Vector2 {
            x: o.x as f32,
            y: o.y as f32,
        }
    }
}
#[cfg(feature = "ggez")]
impl std::convert::From<Point> for ggez::mint::Point3<f32> {
    fn from(o: Point) -> Self {
        ggez::mint::Point3 {
            x: o.x as f32,
            y: o.y as f32,
            z: 0.,
        }
    }
}
#[cfg(feature = "ggez")]
impl std::convert::From<ggez::mint::Point2<f64>> for Point {
    fn from(o: ggez::mint::Point2<f64>) -> Self {
        Point { x: o.x, y: o.y }
    }
}
#[cfg(feature = "ggez")]
impl std::convert::From<ggez::mint::Vector2<f64>> for Point {
    fn from(o: ggez::mint::Vector2<f64>) -> Self {
        Point { x: o.x, y: o.y }
    }
}
#[cfg(feature = "ggez")]
impl std::convert::From<ggez::mint::Point2<f32>> for Point {
    fn from(o: ggez::mint::Point2<f32>) -> Self {
        Point {
            x: o.x as f64,
            y: o.y as f64,
        }
    }
}
#[cfg(feature = "ggez")]
impl std::convert::From<ggez::mint::Vector2<f32>> for Point {
    fn from(o: ggez::mint::Vector2<f32>) -> Self {
        Point {
            x: o.x as f64,
            y: o.y as f64,
        }
    }
}
#[cfg(feature = "ggez")]
impl std::convert::From<ggez::mint::Point3<f32>> for Point {
    fn from(o: ggez::mint::Point3<f32>) -> Self {
        Point {
            x: o.x as f64,
            y: o.y as f64,
        }
    }
}
// impl std::convert::From<Point> for vec2d::Coord {
//     fn from(o: Point) -> vec2d::Coord {
//         vec2d::Coord {
//             x: o.x as u64,
//             y: o.y as u64,
//         }
//     }
// }
// impl std::convert::From<vec2d::Coord> for Point {
//     fn from(o: vec2d::Coord) -> Self {
//         Point {
//             x: o.x as f64,
//             y: o.y as f64,
//         }
//     }
// }
impl std::convert::From<(u32, u32)> for Point {
    fn from(other: (u32, u32)) -> Self {
        Point {
            x: other.0 as f64,
            y: other.1 as f64,
        }
    }
}
impl std::convert::From<(f32, f32)> for Point {
    fn from(other: (f32, f32)) -> Self {
        Point {
            x: other.0 as f64,
            y: other.1 as f64,
        }
    }
}
impl std::convert::From<(f64, f64)> for Point {
    fn from(other: (f64, f64)) -> Self {
        Point {
            x: other.0,
            y: other.1,
        }
    }
}
impl std::convert::From<Point> for (f64, f64) {
    fn from(other: Point) -> Self {
        (other.x, other.y)
    }
}

impl std::convert::From<(u64, u64)> for Point {
    fn from(other: (u64, u64)) -> Self {
        Point {
            x: other.0 as f64,
            y: other.1 as f64,
        }
    }
}
impl std::convert::From<Point> for (u64, u64) {
    fn from(other: Point) -> Self {
        (other.x as u64, other.y as u64)
    }
}

impl std::convert::From<f64> for Point {
    fn from(other: f64) -> Self {
        Point::new(other, other)
    }
}

// +
impl std::ops::Add<Self> for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Add<f64> for Point {
    type Output = Self;

    fn add(self, other: f64) -> Point {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

// -
impl std::ops::Sub<Self> for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl std::ops::Sub<f64> for Point {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

// /
impl std::ops::Div<Point> for Point {
    type Output = Self;

    fn div(self, other: Point) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}
impl std::ops::DivAssign for Point {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

// *
impl std::ops::Mul<Point> for Point {
    type Output = Self;

    fn mul(self, other: Point) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
impl std::ops::MulAssign for Point {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

// -Self
impl std::ops::Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[test]
fn point_comparison() {
    assert!(Point::new(0., 0.) < Point::new(1., 1.));

    assert!(Point::new(0., 0.) < Point::new(0., 1.));

    assert!(Point::new(0., 0.) < Point::new(1., 0.));

    assert!(Point::new(1., 0.) < Point::new(1., 1.));

    assert!(Point::new(1., 1.) == Point::new(1., 1.));

    // this is kinda weird but understandable
    // X dominates over Y
    assert!(Point::new(1., 0.) > Point::new(0., 1.));
}

#[test]
fn test_angle_math() {
    let a = 3.14159265358979323846 * 0.5;
    let mut l1 = super::Line::new(Point::ZERO, Point::ONE);
    l1.rotate(a);
    let end_pt = l1.1;

    assert_eq!(Point::new_rotated(Point::ZERO, Point::ONE, a), end_pt)
}
