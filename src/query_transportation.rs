use duckdb::Connection;
use geo_types::Geometry;
use geozero::wkb::FromWkb;
use geozero::wkb::WkbDialect;

use crate::transportation::RoadClass;
use crate::transportation::Segment;
use crate::transportation::{line_string_road, Road};

pub struct TransportationQueryParams {
    pub from_string: String,
    pub k: f64,
    pub limit: Option<u32>,
    pub translate: [f64; 2],
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
    let limit: String = match params.limit {
        Some(l) => format!("LIMIT {}", l),
        None => String::from(""),
    };
    let mut stmt = conn
        .prepare(&format!(
            "SELECT
                id,
                ST_GeomFromWkb(geometry) AS geometry,
                road,
                level
                FROM {from} {limit}"
        ))
        .unwrap();
    #[derive(Debug)]
    struct DbSegment {
        // id: String,
        geom: Vec<u8>,
        road: Option<String>,
        level: Option<u32>,
        // connectors: Option<String>,
    }

    let now = std::time::Instant::now();
    let query_iter = stmt
        .query_map([], |row| {
            Ok(DbSegment {
                // id: row.get(0)?,
                geom: row.get(1)?,
                road: row.get(2)?,
                level: row.get(3)?,
                // connectors: row.get(2)?,
            })
        })
        .unwrap();
    println!("{:?}", now.elapsed());
    let mut segments: Vec<Segment> = vec![];
    for item in query_iter {
        let item = item.unwrap();
        let raw = item.geom;
        // MAGIC TO GET ARRAY THAT WORKS, COMPARED TO BINARY FROM PARQUET DIRECTLY
        // 0, 1, 104, 0, 0, 0, 0, 0, 1
        let raw = &raw[9..];
        let prefix: [u8; 2] = [1, 2];
        let raw = [prefix.as_slice(), &raw].concat();
        let mut rdr = std::io::Cursor::new(raw);
        let g = Geometry::from_wkb(&mut rdr, WkbDialect::Wkb);
        match g {
            Ok(g) => match g {
                Geometry::LineString(line_string) => {
                    if let Some(road) = &item.road {
                        // dbg!(&road);
                        // dbg!(&item.level);
                        let (translate, line) =
                            line_string_road(line_string, params.k, params.translate);
                        let road_parsed: Road = serde_json::from_str(road).expect("road");
                        let road_class: RoadClass = RoadClass::from_string(&road_parsed.class);
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
