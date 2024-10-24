pub enum BaseShape {
    Circle(f64), // radius
    Rect(f64),   // width or height, a square then
}

pub struct Arc {
    base_shape: BaseShape,
    fraction: f64,
}

impl Arc {
    pub fn new(base_shape: BaseShape, fraction: f64) -> Self {
        Self {
            base_shape,
            fraction,
        }
    }

    pub fn points(&self) -> Vec<crate::Point> {
        match self.base_shape {
            BaseShape::Circle(radius) => {
                // Might be a good idea to not draw a point for every angle degree,
                // maybe add a

                // #[rustfmt::skip]
                [
                    (fix_angle(360. * -self.fraction) as i32..=360)
                        .step_by(5)
                        .map(|i: i32| {
                            crate::line::rotate(
                                crate::Line::new(
                                    crate::Point::ZERO,
                                    crate::Point::new(radius / 2., 0.),
                                ),
                                (i as f64 - 90.).to_radians(),
                            )
                            .1
                        })
                        .collect::<Vec<crate::Point>>(),
                    vec![crate::Point::new(0., -(radius / 2.)), crate::Point::ZERO],
                ]
                .concat()
            }
            #[cfg(feature = "geo")]
            BaseShape::Rect(width) => {
                use geo::algorithm::bool_ops::BooleanOps;
                use geo::coord;

                let width = width as i32;

                let end_angle = fix_angle(360. * -self.fraction);

                // debug!("{end_angle}");

                let rect = geo::geometry::Rect::new(
                    coord! {x: -(width /2) as f64, y: -(width /2)as f64},
                    coord! {x: (width /2) as f64, y: (width /2)as f64},
                )
                .to_polygon();

                /*
                    Polygon {
                        exterior:
                            LineString([
                                Coord { x: -50.0, y: -50.0 },
                                Coord { x: -50.0, y: 50.0 }
                                Coord { x: 50.0, y: 50.0 },
                                Coord { x: 50.0, y: -50.0 },
                                Coord { x: -50.0, y: -50.0 }
                            ]),
                        interiors:
                            []
                    }
                */
                // debug!("{rect:?}");
                // using pow here is wrong
                let p1 = crate::Point::new(0., -width as f64 * 2f64.sqrt());
                let p2 = crate::line::rotate(
                    crate::Line::new(
                        crate::Point::ZERO,
                        crate::Point::new(width as f64 * 2f64.sqrt(), 0.),
                    ),
                    (end_angle - 90.).to_radians(),
                )
                .1;

                // as TruelyFalse said,
                /*
                    if you want to be more exact (and if your angles subtend an inner angle greater than 180deg
                    you'll need to do this anyways), you can construct a polygon that connects
                    to the corners of the shape's bounding box
                */
                // but this require to dynamicly chose what corners to use, depending on the angle
                // as the goal is to make (spell?) timers, we'll take the classic route (0, -1), a clock,

                let mut points = vec![coord! {x: 0., y:0.}, coord! {x: p1.x, y: p1.y}];

                // if/else of doom
                if end_angle >= 45.0 {
                    points.push(*rect.exterior().0.get(3).unwrap())
                } else {
                    points.push(coord! {x: p2.x, y: p2.y})
                }
                if end_angle >= 135.0 {
                    points.push(*rect.exterior().0.get(2).unwrap())
                } else {
                    points.push(coord! {x: p2.x, y: p2.y})
                }
                if end_angle >= 225.0 {
                    points.push(*rect.exterior().0.get(1).unwrap())
                } else {
                    points.push(coord! {x: p2.x, y: p2.y})
                }
                if end_angle >= 315.0 {
                    points.push(*rect.exterior().0.get(0).unwrap())
                } else {
                    points.push(coord! {x: p2.x, y: p2.y})
                }
                if end_angle >= 360.0 {
                    points.push(*rect.exterior().0.get(3).unwrap())
                } else {
                    points.push(coord! {x: p2.x, y: p2.y})
                }

                let anti = geo::geometry::Polygon::new(geo::LineString::new(points), vec![]);

                let pt_lst = rect
                    .difference(&anti)
                    .iter()
                    .flat_map(|poly| poly.exterior().clone().into_points())
                    .map(|p| crate::Point::new(p.x(), p.y()))
                    .collect::<Vec<crate::Point>>();

                pt_lst
            }
            #[cfg(not(feature = "geo"))]
            BaseShape::Rect(_) => {
                unimplemented!("You need to enable the geo feature")
            }
        }
    }
}

fn fix_angle(mut a: f64) -> f64 {
    while a < 0. {
        a += 360.
    }

    while a > 360. {
        a -= 360.
    }
    a
}
