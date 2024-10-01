#[macro_use]
extern crate log;

pub mod arc;
pub mod circle;
pub mod collision;
pub mod interpolation;
pub mod line;
pub mod point;
pub mod rect;
pub mod vec2d;

pub use arc::Arc;
pub use circle::Circle;
pub use line::Line;
pub use point::Point;
pub use rect::Rect;
pub use vec2d::Vec2D;

pub type Vec2 = Point;

pub fn clamp<T: std::cmp::PartialOrd>(nbr: T, min: T, max: T) -> T {
    if nbr < min {
        min
    } else if nbr > max {
        max
    } else {
        nbr
    }
}

pub fn get_distance(pt1: &Point, pt2: &Point) -> f64 {
    ((pt1.x - pt2.x).powf(2.) + (pt1.y - pt2.y).powf(2.)).sqrt()
}

// pub fn get_map_index_from_pos(pos: Point) -> (u64, u64) {
//     use crate::game::map::TILE_SIZE;

//     ((pos.x / TILE_SIZE) as u64, (pos.y / TILE_SIZE) as u64)

//     // (0, 0)
// }
