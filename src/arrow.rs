use bevy_prototype_lyon::geometry::Geometry;
use bevy_prototype_lyon::prelude::tess::geom::euclid::{Box2D, Size2D};
use bevy_prototype_lyon::prelude::tess::geom::{Point, point};
use bevy_prototype_lyon::prelude::tess::path::path::Builder;
use bevy_prototype_lyon::prelude::tess::path::{Polygon, Winding};

#[derive(Debug, Clone, PartialEq)]
pub struct Arrow {
    pub rectangle_length: f32,
    pub rectangle_width: f32,
    pub triangle_offset: f32,
    pub triangle_height: f32,
}

impl Geometry for Arrow {
    fn add_geometry(&self, b: &mut Builder) {
        let rectangle_origin = Point::new(-self.rectangle_width/2., 0.);

        b.add_rectangle(
            &Box2D::from_origin_and_size(rectangle_origin, Size2D::new(self.rectangle_width, self.rectangle_length)),
            Winding::Positive
        );
        b.add_polygon(
            Polygon {
                points: &[
                    point(-self.triangle_offset - self.rectangle_width/2., self.rectangle_length),
                    point(self.triangle_offset + self.rectangle_width/2., self.rectangle_length),
                    point(0., self.rectangle_length + self.triangle_height)
                ],
                closed: true
            }
        );
    }
}
