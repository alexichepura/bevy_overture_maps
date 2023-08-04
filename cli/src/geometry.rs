use geo_types::Geometry;
use geozero::wkb::{FromWkb, WkbDialect};

pub fn check_wkb(bytes_array: &[u8]) {
    let mut rdr = std::io::Cursor::new(bytes_array);
    let g = Geometry::from_wkb(&mut rdr, WkbDialect::Wkb);
    dbg!(g);
}
