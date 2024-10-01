/*
    NOTE:
        IM REALLY BAD AT MATHS, EVERYTHING HERE IS REALLY SIMPLE OR FOUND ELSEWHERE
*/

#[derive(Debug, Copy, Clone)]
pub enum CollideDirection {
    Up,
    Down,
    Left,
    Right,
}

pub fn rect_rect_no_r(r1: &super::Rect, r2: &super::Rect) -> bool {
    // fix this, .topleft should be replaced as points[0]

    // let is_collison =
    //     r1.x < r2.x + r2.w && r1.x + r1.h > r2.x && r1.y < r2.y + r2.h && r1.h + r1.y > r2.y;

    // is_collison
    r1.aa_topleft().x < r2.aa_topleft().x + r2.width()
        && r1.aa_topleft().x + r1.height() > r2.aa_topleft().x
        && r1.aa_topleft().y < r2.aa_topleft().y + r2.height()
        && r1.height() + r1.aa_topleft().y > r2.aa_topleft().y
}

pub fn rect_rect(r1: &super::Rect, r2: &super::Rect) -> bool {
    // check if r1 is inside r2
    for point in r1.r_points() {
        if point_rect(&point, r2) {
            return true;
        }
    }

    // check if r2 is inside r1
    for point in r2.r_points() {
        if point_rect(&point, r1) {
            return true;
        }
    }

    // This is for cases like crosses
    for r1l in r1.r_lines() {
        for r2l in r2.r_lines() {
            if line_line(&r1l, &r2l) {
                return true;
            }
        }
    }
    // let r1p = r1.points();
    // let r2p = r2.points();
    // r1p[0].x < r2p[2].x && r1p[2].x > r2p[2].x && r1p[0].y < r2p[2].y && r1p[2].y > r2p[0].y
    // ^this is working but not for the case that one rect is IN the other
    false
}

// pub fn rect_circle(r: &super::Rect, c: super::Circle) -> bool {
//     let lines = r.aa_lines();
//     for line in lines {
//         if line_circle(line, c) {
//             return true;
//         }
//     }
//     false
// }

pub fn rect_line(r: &super::Rect, l: &super::Line) -> bool {
    let lines = r.aa_lines();
    for line in lines {
        if line_line(&line, l) {
            return true;
        }
    }
    false
}

pub fn line_line(l1: &super::Line, l2: &super::Line) -> bool {
    fn ccw(a: &super::Point, b: &super::Point, c: &super::Point) -> bool {
        (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
    }
    fn intersect(a: &super::Point, b: &super::Point, c: &super::Point, d: &super::Point) -> bool {
        ccw(a, c, d) != ccw(b, c, d) && ccw(a, b, c) != ccw(a, b, d)
    }
    intersect(&l1.0, &l1.1, &l2.0, &l2.1)
}
pub fn line_closest_pt(line: &super::Line, point: &super::Point) -> super::Point {
    let (x1, y1) = (line.0.x, line.0.y);
    let (x2, y2) = (line.1.x, line.1.y);
    let (x3, y3) = (point.x, point.y);
    let (dx, dy) = (x2 - x1, y2 - y1);
    let det = dx * dx + dy * dy;
    let a = (dy * (y3 - y1) + dx * (x3 - x1)) / det;

    let mut new_point = super::Point::new(x1 + a * dx, y1 + a * dy);

    let line_length = super::get_distance(&line.0, &line.1);

    let dist_to_a = super::get_distance(&new_point, &line.0);

    let dist_to_b = super::get_distance(&new_point, &line.1);

    if dist_to_a > line_length && dist_to_a > dist_to_b {
        new_point = line.1;
    }
    if dist_to_b > line_length && dist_to_b > dist_to_a {
        new_point = line.0;
    }

    new_point
}

// pub fn line_circle(line: &super::Line, circle: super::Circle) -> bool {
//     let closest_point = line_closest_pt(line, circle.center);

//     let collision_result = point_in_circle(closest_point, circle);
//     // let result = collision_result{
//     //     CollisionResult::Out => {
//     //         // The line doesn't cross the circle
//     //     }
//     //     CollisionResult::In => {
//     //         // The line does cross the circle
//     //     }
//     //     CollisionResult::Touch => {
//     //         // The line touches the circle
//     //     }
//     // };
//     collision_result
// }

// pub fn circle_circle(circle1: super::Circle, circle2: super::Circle) -> bool {
//     // https://developer.mozilla.org/fr/docs/Games/Techniques/2D_collision_detection
//     // let dx = circle1.center.0 - circle2.center.0;
//     // let dy = circle1.center.1 - circle2.center.1;
//     // let distance = (dx * dx + dy * dy).sqrt();
//     let distance = super::get_distance(circle1.center, circle2.center);

//     distance < circle1.radius + circle2.radius
// }

pub fn point_rect(point: &super::Point, rect: &super::Rect) -> bool {
    // let x1 = rect.topleft().x;
    // let y1 = rect.topleft().y;
    // let x2 = x1 + rect.width();
    // let y2 = y1 + rect.height();
    // x1 < point.x && point.x < x2 && y1 < point.y && point.y < y2

    // https://gamedev.stackexchange.com/questions/128598/collision-detection-point-hitting-a-rotating-rectangle/154294#154294
    // "Note that you don't have to rotate the rectangle.
    // You can use the coordinates before it was rotated.
    // You just have to rotate the point by minus the angle of the rectangle,
    // around the center of rotation of the rectangle."

    // so, as in Vupa the center of rotation of a rect is it's center, just rotate the point by -rect.angle()
    // with rect.center() as 'origin'/'anchor'
    // Visualisation: https://i.stack.imgur.com/vRmRH.png (Image from Heckel's response in the link above)
    let rotated_point = super::Point::new_rotated(rect.center(), *point, -rect.rotation());
    // super::line::rotate(super::Line::new(rect.center(), point), -rect.rotation()).1;

    // and then, just check the rotated point with the non-rotated rect
    let x1 = rect.aa_topleft().x;
    let y1 = rect.aa_topleft().y;
    let x2 = x1 + rect.width();
    let y2 = y1 + rect.height();
    x1 < rotated_point.x && rotated_point.x < x2 && y1 < rotated_point.y && rotated_point.y < y2

    // false
}

// pub fn point_in_circle(point: &super::Point, circle: super::Circle) -> bool {
//     //CollisionResult
//     let dist_point_circle_center = super::get_distance(circle.center, point);

//     // if dist_point_circle_center > circle.radius {
//     //     // The point is outside the circle
//     //     // CollisionResult::Out
//     //     false
//     // } else if dist_point_circle_center < circle.radius {
//     //     // The point is in the circle
//     //     // CollisionResult::In
//     //     true
//     // } else {
//     //     // The point is on the circle ring
//     //     // CollisionResult::Touch
//     //     true
//     // }
//     dist_point_circle_center <= circle.radius
// }
