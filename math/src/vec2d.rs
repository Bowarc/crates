/*
    for mental comprehension
    it can be visualized like that (numbers are indexes)
    [
        [0 , 1 , 2 , 3 ], // this is a row
        [4 , 5 , 6 , 7 ],
        [8 , 9 , 10, 11],
        [12, 13, 14, 15],
        [16, 17, 18, 19],
        [20, 21, 22, 23],
    ]
    + making a list of columns doesn't make any sense
*/
pub struct IndexOutOfBoundsError;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub struct Vec2D<T> {
    elems: Vec<T>,
    size: crate::Vec2,
}

// we could use crate::Point but using u64 is better for iteration
/*
    Index rect vs position rect is a bit anoying so i'll assume taht every given rect is a position rect
*/
#[derive(Debug, PartialEq)]
pub struct Vec2DIterator {
    start: (u64, u64),
    end: (u64, u64),

    current: (u64, u64),
}

impl<T: Clone + std::fmt::Debug> Vec2D<T> {
    pub fn new_from_element(elem: T, size: impl Into<crate::Vec2>) -> Self {
        let size = size.into();
        Self {
            elems: vec![elem; (size.x * size.y) as usize],
            size,
        }
    }
    #[cfg(feature = "ggez")]
    pub fn set_rect_from_elem(
        &mut self,
        rect: crate::Rect,
        elem: T,
        ignore_errors: bool,
    ) -> ggez::GameResult {
        if rect.width() == 0. || rect.height() == 0. {
            return Err(ggez::GameError::CustomError(String::from(
                "Due to common sense, rects of size 0 are not allowed in Vec2D::set_rect_from_elem",
            )));
        }
        for pt in self.rect_iter(rect) {
            // println!("Setting {elem:?} at pt {pt}");
            if self.set(pt, elem.clone()).is_err() && !ignore_errors {
                return Err(ggez::GameError::CustomError(
                    "Could not set data at index: {pt:?}".to_string(),
                ));
            }
        }
        Ok(())
    }

    #[cfg(not(feature = "ggez"))]
    pub fn set_rect_from_elem(
        &mut self,
        rect: crate::Rect,
        elem: T,
        ignore_errors: bool,
    ) -> Result<(), String> {
        if rect.width() == 0. || rect.height() == 0. {
            return Err(String::from(
                "Due to common sense, rects of size 0 are not allowed in Vec2D::set_rect_from_elem",
            ));
        }
        for pt in self.rect_iter(rect) {
            // println!("Setting {elem:?} at pt {pt}");
            if self.set(pt, elem.clone()).is_err() && !ignore_errors {
                return Err(format!("Could not set data at index: {pt:?}"));
            }
        }
        Ok(())
    }
}

impl<T> Vec2D<T> {
    pub fn new_empty() -> Self {
        Self {
            elems: Vec::<T>::new(),
            size: crate::Vec2::ZERO,
        }
    }
    pub fn new_from_vec(base: Vec<T>, size: impl Into<crate::Vec2>) -> Option<Vec2D<T>> {
        let size = size.into();

        if (size.x * size.y) as u64 == base.len() as u64 {
            Some(Self { elems: base, size })
        } else {
            None
        }
    }
    pub fn elems(&self) -> &Vec<T> {
        &self.elems
    }
    pub fn size(&self) -> crate::Point {
        self.size
    }
    pub fn index_from_point(&self, pt: impl Into<crate::Point>) -> u64 {
        let pt = pt.into();

        (pt.y * self.size.x + pt.x) as u64
    }

    pub fn contains_point(&self, pt: impl Into<crate::Point>) -> bool {
        let pt = crate::Point::floored(&pt.into());

        pt.x < self.size.x && pt.y < self.size.y
    }
    pub fn get(&self, pt: impl Into<crate::Point>) -> Option<&T> {
        let pt = crate::Point::floored(&pt.into());

        if self.contains_point(pt) {
            let index = self.index_from_point(pt);

            Some(&self.elems[index as usize])
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, pt: impl Into<crate::Point>) -> Option<&mut T> {
        let pt = crate::Point::floored(&pt.into());
        if self.contains_point(pt) {
            let index = self.index_from_point(pt);
            Some(&mut self.elems[index as usize])
        } else {
            None
        }
    }
    // returns Ok(T) if it succesfully replaced an item
    // return Err(()) if it failled to place the item in the elems list
    pub fn set(
        &mut self,
        pt: impl Into<crate::Point>,
        elem: T,
    ) -> Result<T, IndexOutOfBoundsError> {
        let pt = pt.into();
        if self.contains_point(pt) {
            let index = self.index_from_point(pt);

            Ok(std::mem::replace(
                self.elems.get_mut(index as usize).unwrap(),
                elem,
            ))
        } else {
            Err(IndexOutOfBoundsError)
        }
    }
    pub fn iter(&self) -> Vec2DIterator {
        self.rect_iter(crate::Rect::new(crate::Point::ZERO, self.size, 0.))
    }

    pub fn rect_iter(&self, position_rect: crate::Rect) -> Vec2DIterator {
        // Here the -1 is usefull to transform the position rect into a index rect
        Vec2DIterator::new(crate::Rect::new(
            position_rect.aa_topleft(),
            position_rect.size() - 1.,
            0.,
        ))
    }

    pub fn rect_iter_clamped(&self, rect: crate::Rect) -> Vec2DIterator {
        // clamp the rect to the vec2d's borders, to be sure that a future `.get()` call doesn't fail
        let x = crate::clamp(rect.aa_topleft().x, 0., self.size.x);
        let y = crate::clamp(rect.aa_topleft().y, 0., self.size.y);

        let w = crate::clamp(rect.width(), 0., self.size.x - x);
        let h = crate::clamp(rect.height(), 0., self.size.y - y);

        let clamped_rect = crate::Rect::new(crate::Point::new(x, y), crate::Vec2::new(w, h), 0.);

        if clamped_rect != rect {
            // println!("Clamped rect from {rect} to {clamped_rect}");
        }
        self.rect_iter(clamped_rect)
    }
}

impl Vec2DIterator {
    pub fn new(rect: crate::Rect) -> Self {
        if rect.rotation() != 0. {
            warn!("Rotated rect cannot be used to index map");
        }

        Self {
            start: rect.aa_topleft().into(),
            end: rect.aa_botright().into(),
            current: rect.aa_topleft().into(),
        }
    }
}

impl Iterator for Vec2DIterator {
    type Item = crate::Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.1 > self.end.1 {
            return None;
        }
        let current = self.current;

        let mut next = self.current;

        if next.0 < self.end.0 {
            next.0 += 1;
        } else {
            next.0 = self.start.0;
            next.1 += 1
        }

        self.current = next;

        Some(current.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_iter() {
        let vec = Vec2D::new_from_element(0, crate::Vec2::new(7., 6.));

        for pt in vec.iter() {
            assert!(vec.get(pt).is_some());
        }
    }

    #[test]
    fn clamped_iter() {
        let vec = Vec2D::new_from_element(0, crate::Vec2::new(10., 10.));

        assert_eq!(
            vec.iter(),
            vec.rect_iter_clamped(crate::Rect::new(
                crate::Point::ZERO,
                crate::Vec2::new(11., 10.),
                0.
            ))
        );
    }

    #[test]
    fn set_rect_from_elem() {
        let mut vec = Vec2D::new_from_element(0, crate::Vec2::new(10., 10.));

        let rect = crate::Rect::new(crate::Point::new(6., 4.), crate::Vec2::new(3., 4.), 0.);

        // this should not fail
        vec.set_rect_from_elem(rect, 1, false).unwrap();

        assert_eq!(vec.get(rect.aa_topleft()), Some(&1));
        assert_eq!(
            vec.get((rect.aa_botright().x - 1., rect.aa_botright().y - 1.)),
            Some(&1)
        );
    }

    #[test]
    fn integrity() {
        let mut vec = Vec2D::new_from_element(0, crate::Vec2::new(10., 10.));

        vec.elems[1] = 1;
        vec.elems[vec.size.y as usize] = 2;

        assert_eq!(vec.get(crate::Point::new(1., 0.)), Some(&1));
        assert_eq!(vec.get(crate::Point::new(0., 1.)), Some(&2));
    }
}
