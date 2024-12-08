// #![allow(unused)]
mod inner;
use inner::*;
mod cache;
use cache::*;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(from = "InnerRect")
)]
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    inner: InnerRect,
    
    #[cfg_attr(feature = "serde", serde(skip_serializing))]   
    cache: PointCache,
}

impl Rect {
    //About caching, idk if i'll re-implement it now that i found the math to make things cleanly
    pub fn new(
        topleft: impl Into<crate::Point>,
        size: impl Into<crate::Vec2>,
        rotation: impl Into<f64>,
    ) -> Self {
        let inner = InnerRect::new(topleft, size, rotation);
        let cache = PointCache::new(inner);

        Self { inner, cache }
    }
    pub fn new_from_center(
        center: impl Into<super::Point>,
        size: impl Into<super::Point>,
        rotation: impl Into<f64>,
    ) -> Self {
        let center = center.into();
        let size = size.into();

        let topleft = center - size * 0.5;

        Self::new(topleft, size, rotation)
    }

    /// Check if the rect is axis aligned
    pub fn is_aa(&self) -> bool {
        self.rotation() == 0.
    }
    fn update_cache(&mut self) {
        self.cache = PointCache::new(self.inner);
        // debug!("Update cache");
    }
    // fn validate_cache(&mut self) {
    //     if self.rotation() != 0. {
    //         self.update_cache()
    //     }
    // }
    pub fn center(&self) -> crate::Point {
        self.inner.center()
    }
    pub fn set_center(&mut self, new_center: impl Into<crate::Point>) {
        let new_center = new_center.into();
        if new_center == self.center() {
            return;
        }
        self.inner.set_center(new_center);
        if !self.is_aa() {
            self.update_cache()
        }
    }
    pub fn size(&self) -> crate::Vec2 {
        self.inner.size()
    }
    pub fn set_size(&mut self, new_size: impl Into<crate::Vec2>) {
        let new_size = new_size.into();
        if self.size() == new_size {
            return;
        }
        self.inner.set_size(new_size);
        if !self.is_aa() {
            self.update_cache()
        }
    }
    pub fn width(&self) -> f64 {
        self.size().x
    }
    pub fn set_width(&mut self, new_width: impl Into<f64>) {
        let new_size = crate::Vec2::new(new_width.into(), self.height());
        self.set_size(new_size);
    }
    pub fn height(&self) -> f64 {
        self.size().y
    }
    pub fn set_height(&mut self, new_height: impl Into<f64>) {
        let new_size = crate::Vec2::new(self.width(), new_height.into());
        self.set_size(new_size);
    }
    pub fn rotation(&self) -> f64 {
        self.inner.rotation()
    }
    pub fn set_rotation(&mut self, new_rotation: impl Into<f64>) {
        let new_rotation = new_rotation.into();
        if self.rotation() == new_rotation {
            return;
        }
        self.inner.set_rotation(new_rotation);
        if !self.is_aa() {
            self.update_cache()
        }
    }
}

// Axis aligned impl
impl Rect {
    pub fn aa_topleft(&self) -> crate::Point {
        self.inner.aa_topleft()
    }
    pub fn aa_topright(&self) -> crate::Point {
        self.inner.aa_topright()
    }
    pub fn aa_botright(&self) -> crate::Point {
        self.inner.aa_botright()
    }
    pub fn aa_botleft(&self) -> crate::Point {
        self.inner.aa_botleft()
    }
    pub fn aa_lines(&self) -> [crate::Line; 4] {
        self.inner.aa_lines()
    }
    pub fn aa_points(&self) -> [crate::Point; 4] {
        self.inner.aa_points()
    }
    pub fn aa_points5(&self) -> [crate::Point; 5] {
        self.inner.aa_points5()
    }
}

/// Rotated impl
// hmm, in a perfect world i'd love to only update on fetch when cache isn't updated, but that would require fetch function to query self as mutable.
// Idk if that is a rly good idea
// i'll create both and try
// After testing.. It appear to not be better, maybe too much useless checks
impl Rect {
    pub fn r_topleft(&self) -> crate::Point {
        if self.is_aa() {
            return self.aa_topleft();
        }
        self.cache.r_topleft()
    }
    pub fn r_topright(&self) -> crate::Point {
        if self.is_aa() {
            return self.aa_topright();
        }
        self.cache.r_topright()
    }
    pub fn r_botright(&self) -> crate::Point {
        if self.is_aa() {
            return self.aa_botright();
        }
        self.cache.r_botright()
    }
    pub fn r_botleft(&self) -> crate::Point {
        if self.is_aa() {
            return self.aa_botleft();
        }
        self.cache.r_botleft()
    }
    pub fn r_lines(&self) -> [crate::Line; 4] {
        if self.is_aa() {
            return self.aa_lines();
        }
        self.cache.r_lines()
    }
    pub fn r_points(&self) -> [crate::Point; 4] {
        if self.is_aa() {
            return self.aa_points();
        }
        self.cache.r_points()
    }
    pub fn r_points5(&self) -> [crate::Point; 5] {
        if self.is_aa() {
            return self.aa_points5();
        }
        self.cache.r_points5()
    }
}

impl std::cmp::PartialEq for Rect {
    fn eq(&self, other: &Rect) -> bool {
        self.inner == other.inner
    }
}

impl std::ops::Add<crate::Point> for Rect {
    type Output = Rect;
    fn add(self, pt: crate::Point) -> Rect {
        Rect::new(
            (self.aa_topleft().x + pt.x, self.aa_topleft().y + pt.y),
            (self.width(), self.height()),
            self.rotation(),
        )
    }
}

impl std::ops::Sub<crate::Point> for Rect {
    type Output = Rect;
    fn sub(self, pt: crate::Point) -> Rect {
        Rect::new(
            (self.aa_topleft().x - pt.x, self.aa_topleft().y - pt.y),
            (self.width(), self.height()),
            self.rotation(),
        )
    }
}

#[cfg(feature ="ggez")]
impl From<Rect> for ggez::graphics::Rect {
    fn from(rect: Rect) -> ggez::graphics::Rect {
        ggez::graphics::Rect {
            x: rect.aa_topleft().x as f32,
            y: rect.aa_topleft().y as f32,
            w: rect.width() as f32,
            h: rect.height() as f32,
        }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "crate::Rect, x{} y{}, w{} h{}, a{}",
            self.aa_topleft().x,
            self.aa_topleft().y,
            self.width(),
            self.height(),
            self.rotation()
        )
    }
}

impl From<InnerRect> for Rect{
    fn from(inner: InnerRect) -> Self {
        Self{
            inner,
            cache: PointCache::new(inner)
        }
    }
}

#[test]
fn integrity() {
    let mut rect = Rect::new((12., 56.), (689., 2.), 6.);

    assert_eq!(rect.r_topleft(), rect.inner.r_topleft());
    assert_eq!(rect.r_topleft(), rect.cache.r_topleft());

    rect.set_rotation(9.);

    assert_eq!(rect.r_topleft(), rect.inner.r_topleft());
    assert_eq!(rect.r_topleft(), rect.cache.r_topleft());
}
