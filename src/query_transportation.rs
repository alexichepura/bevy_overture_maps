use duckdb::Connection;
use geo_types::Geometry;
use geozero::wkb::FromWkb;
use geozero::wkb::WkbDialect;

use crate::KxyGeodesic;
use crate::transportation::RoadClass;
use crate::transportation::Segment;
use crate::transportation::line_string_road;

pub struct TransportationQueryParams {
    pub from_string: String,
    pub k: KxyGeodesic,
    pub limit: Option<u32>,
    pub center: [f64; 2],
}

// https://docs.overturemaps.org/reference/transportation/segment
// https://github.com/alexichepura/overture_maps_rs/issues/1

pub fn query_transportation(params: TransportationQueryParams) -> Vec<Segment> {
    let path = "./data.duckdb";
    let conn = Connection::open(&path).unwrap();
    conn.execute_batch("INSTALL httpfs; LOAD httpfs;").unwrap();
    conn.execute_batch("INSTALL spatial; LOAD spatial;")
        .unwrap();
    let from = params.from_string;
    let limit_clause = match params.limit {
        Some(l) => format!("LIMIT {}", l),
        None => String::from(""),
    };
    let query = if limit_clause.is_empty() {
        format!("SELECT id, ST_AsWKB(geometry) as geometry, class FROM {from}")
    } else {
        format!("SELECT id, ST_AsWKB(geometry) as geometry, class FROM {from} {}", limit_clause)
    };
    let mut stmt = conn.prepare(&query).unwrap();
    #[derive(Debug)]
    struct DbSegment {
        // id: String,
        geom: Vec<u8>,
        class: Option<String>,
        // level_rules: Option<String>,
        // connectors: Option<String>,
    }

    let now = std::time::Instant::now();
    let query_iter = match stmt.query_map([], |row| {
        Ok(DbSegment {
            // id: row.get(0)?,
            geom: row.get(1)?,
            class: row.get(2)?,
        })
    }) {
        Ok(iter) => iter,
        Err(e) => {
            eprintln!("Failed to create query map: {:?}", e);
            return Vec::new();
        }
    };
    println!("{:?}", now.elapsed());
    let mut segments: Vec<Segment> = vec![];
    for item in query_iter {
        let item = item.unwrap();
        let raw = item.geom;
        let mut rdr = std::io::Cursor::new(raw);
        let g = Geometry::from_wkb(&mut rdr, WkbDialect::Wkb);
        match g {
            Ok(g) => match g {
                Geometry::LineString(line_string) => {
                    if let Some(class) = &item.class {
                        let (translate, line) =
                            line_string_road(line_string, params.k, params.center);
                        let road_class: RoadClass = RoadClass::from_string(class);
                        let segment = Segment {
                            translate,
                            line,
                            k: params.k,
                            road_class,
                        };
                        segments.push(segment);
                    }
                }
                not_line_string => {
                    dbg!(&not_line_string);
                }
            },
            Err(e) => {
                dbg!(e);
            }
        }
    }

    segments
}
