use super::InnerRect;

#[derive(Clone, Copy, Debug)]
pub struct PointCache {
    points: [crate::Point; 4],
}

impl PointCache {
    pub fn new(inner_rect: impl Into<InnerRect>) -> Self {
        Self {
            points: inner_rect.into().r_points(),
        }
    }
    pub fn r_topleft(&self) -> crate::Point {
        *self.points.get(0).unwrap()
    }
    pub fn r_topright(&self) -> crate::Point {
        *self.points.get(1).unwrap()
    }
    pub fn r_botright(&self) -> crate::Point {
        *self.points.get(2).unwrap()
    }
    pub fn r_botleft(&self) -> crate::Point {
        *self.points.get(3).unwrap()
    }
    pub fn r_lines(&self) -> [crate::Line; 4] {
        [
            crate::Line::new(self.r_topleft(), self.r_topright()),
            crate::Line::new(self.r_topright(), self.r_botright()),
            crate::Line::new(self.r_botright(), self.r_botleft()),
            crate::Line::new(self.r_botleft(), self.r_topleft()),
        ]
    }
    pub fn r_points(&self) -> [crate::Point; 4] {
        [
            self.r_topleft(),
            self.r_topright(),
            self.r_botright(),
            self.r_botleft(),
        ]
    }
    pub fn r_points5(&self) -> [crate::Point; 5] {
        [
            self.r_topleft(),
            self.r_topright(),
            self.r_botright(),
            self.r_botleft(),
            self.r_topleft(),
        ]
    }
}
