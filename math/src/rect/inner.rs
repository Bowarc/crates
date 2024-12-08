#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InnerRect {
    center: crate::Point,
    size: crate::Vec2,
    rotation: f64,
}

impl InnerRect {
    pub fn new(
        topleft: impl Into<crate::Point>,
        size: impl Into<crate::Vec2>,
        rotation: impl Into<f64>,
    ) -> Self {
        let topleft = topleft.into();
        let size = size.into();
        let rotation = rotation.into();

        Self {
            center: topleft + size * 0.5,
            size,
            rotation,
        }
    }

    pub fn center(&self) -> crate::Point {
        self.center
    }
    pub fn set_center(&mut self, new_center: impl Into<crate::Point>) {
        self.center = new_center.into()
    }
    pub fn size(&self) -> crate::Vec2 {
        self.size
    }
    pub fn set_size(&mut self, new_size: impl Into<crate::Vec2>) {
        self.size = new_size.into()
    }
    pub fn rotation(&self) -> f64 {
        self.rotation
    }
    pub fn set_rotation(&mut self, new_rotation: impl Into<f64>) {
        self.rotation = new_rotation.into()
    }
    /// Axis aligned top left point
    pub fn aa_topleft(&self) -> crate::Point {
        crate::Point::new(
            self.center.x - self.size.x * 0.5,
            self.center.y - self.size.y * 0.5,
        )
    }
    /// Axis aligned top right point
    pub fn aa_topright(&self) -> crate::Point {
        crate::Point::new(
            self.center.x + self.size.x * 0.5,
            self.center.y - self.size.y * 0.5,
        )
    }
    /// Axis aligned bot right point
    pub fn aa_botright(&self) -> crate::Point {
        crate::Point::new(
            self.center.x + self.size.x * 0.5,
            self.center.y + self.size.y * 0.5,
        )
    }
    /// Axis aligned bot left point
    pub fn aa_botleft(&self) -> crate::Point {
        crate::Point::new(
            self.center.x - self.size.x * 0.5,
            self.center.y + self.size.y * 0.5,
        )
    }
    /// Axis aligned lines
    pub fn aa_lines(&self) -> [crate::Line; 4] {
        [
            crate::Line::new(self.aa_topleft(), self.aa_topright()),
            crate::Line::new(self.aa_topright(), self.aa_botright()),
            crate::Line::new(self.aa_botright(), self.aa_botleft()),
            crate::Line::new(self.aa_botleft(), self.aa_topleft()),
        ]
    }
    /// Axis aligned points
    pub fn aa_points(&self) -> [crate::Point; 4] {
        [
            self.aa_topleft(),
            self.aa_topright(),
            self.aa_botright(),
            self.aa_botleft(),
        ]
    }
    /// Axis aligned 5 points
    pub fn aa_points5(&self) -> [crate::Point; 5] {
        [
            self.aa_topleft(),
            self.aa_topright(),
            self.aa_botright(),
            self.aa_botleft(),
            self.aa_topleft(),
        ]
    }
    /// Rotated top left
    pub fn r_topleft(&self) -> crate::Point {
        crate::Point::new_rotated(self.center, self.aa_topleft(), self.rotation)
    }
    /// Rotated top right
    pub fn r_topright(&self) -> crate::Point {
        crate::Point::new_rotated(self.center, self.aa_topright(), self.rotation)
    }
    /// Rotated bot right
    pub fn r_botright(&self) -> crate::Point {
        crate::Point::new_rotated(self.center, self.aa_botright(), self.rotation)
    }
    /// Rotated bot left
    pub fn r_botleft(&self) -> crate::Point {
        crate::Point::new_rotated(self.center, self.aa_botleft(), self.rotation)
    }
    /// Rotated lines
    pub fn r_lines(&self) -> [crate::Line; 4] {
        [
            crate::Line::new(self.r_topleft(), self.r_topright()),
            crate::Line::new(self.r_topright(), self.r_botright()),
            crate::Line::new(self.r_botright(), self.r_botleft()),
            crate::Line::new(self.r_botleft(), self.r_topleft()),
        ]
    }
    /// Rotated points
    pub fn r_points(&self) -> [crate::Point; 4] {
        [
            self.r_topleft(),
            self.r_topright(),
            self.r_botright(),
            self.r_botleft(),
        ]
    }
    /// Rotated points 5
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
