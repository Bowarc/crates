#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line(pub super::Point, pub super::Point);
impl Line {
    pub fn new(p0: super::Point, p1: super::Point) -> Self {
        Self(p0, p1)
    }
    pub fn new_rotated(origin: super::Point, range: f64, angle: f64) -> Self {
        let non_rotated = Self::new(origin, origin + super::Point::new(range, 0.));

        rotate(non_rotated, angle)
    }
    #[rustfmt::skip]
    pub fn rotate(&mut self, angle: f64) {
        *self = Line::new(
            self.0,
            super::Point::new(
                self.0.x
                    + angle.cos() * (self.1.x - self.0.x)
                    - angle.sin() * (self.1.y - self.0.y),
                self.0.y
                    + angle.sin() * (self.1.x - self.0.x)
                    + angle.cos() * (self.1.y - self.0.y),
            ),
        )
    }
    // I don't remember it being tested
    pub fn center(&self) -> super::Point {
        super::Point::new(
            std::cmp::max(self.0.x as i32, self.1.x as i32) as f64,
            std::cmp::min(self.0.y as i32, self.1.y as i32) as f64,
        )
    }
}

pub fn rotate(mut line: Line, angle: f64) -> Line {
    line.rotate(angle);
    line
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[test]
fn line_rotation() {
    let origin = super::Point::new(4153., 24.);
    let range = 20.;
    let angle = 0.1526;

    let ray1 = rotate(
        Line::new(origin, origin + super::Point::new(range, 0.)),
        angle,
    );

    let ray2 = Line::new_rotated(origin, range, angle);

    assert_eq!(ray1, ray2);
}
