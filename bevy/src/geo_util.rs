use geo::{GeodesicDistance, Vector2DOps};
use geo_types::Coord;
use std::ops::{Add, Neg};

pub fn geodesic_to_coord(base: Coord) -> f64 {
    let base_d: Coord = Coord {
        x: base.x + 0.001,
        y: base.y + 0.001,
    };
    let p1 = geo::Point(base);
    let p2 = geo::Point(base_d);
    let geodesic_distance = p1.geodesic_distance(&p2);
    let coord_distance = base.add(base_d.neg()).magnitude();
    let k = geodesic_distance / coord_distance;
    k
}
