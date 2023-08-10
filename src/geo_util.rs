use geo::{GeodesicDistance, Vector2DOps};
use geo_types::Coord;
use std::ops::{Add, Neg};

pub type KxyGeodesic = [f64; 2];
pub fn geodesic_to_coord(base: Coord) -> KxyGeodesic {
    let base_d: Coord = Coord {
        x: base.x + 0.001,
        y: base.y,
    };
    let p1 = geo::Point(base);
    let p2 = geo::Point(base_d);
    let geodesic_distance = p1.geodesic_distance(&p2);
    let coord_distance = base.add(base_d.neg()).magnitude();
    let k = geodesic_distance / coord_distance;

    let base_dy: Coord = Coord {
        x: base.x,
        y: base.y + 0.001,
    };
    let p1y = geo::Point(base);
    let p2y = geo::Point(base_dy);
    let geodesic_distance_y = p1y.geodesic_distance(&p2y);
    let coord_distance_y = base.add(base_dy.neg()).magnitude();
    let k_y = geodesic_distance_y / coord_distance_y;

    let kk: KxyGeodesic = [k, k_y];
    dbg!(kk);

    kk
}
