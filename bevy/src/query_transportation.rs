use duckdb::Connection;
use geo_types::Geometry;
use geozero::wkb::FromWkb;
use geozero::wkb::WkbDialect;

use crate::transportation::line_string_base;
use crate::transportation::line_string_road;
use crate::transportation::BevyTransportation;

pub struct TransportationQueryParams {
    pub limit: usize,
    pub where_string: String,
    pub from_string: String,
}

pub fn query_transportation(params: TransportationQueryParams) -> Vec<BevyTransportation> {
    let path = "./data.duckdb";
    let conn = Connection::open(&path).unwrap();
    conn.execute_batch("INSTALL httpfs; LOAD httpfs;").unwrap();
    conn.execute_batch("INSTALL spatial; LOAD spatial;")
        .unwrap();
    let limit = params.limit;
    let where_string = params.where_string;
    let from = params.from_string;
    println!("statement for transportation - start");
    let mut stmt = conn
        // .prepare(&format!(
        //     "SELECT
        //         id,
        //         subtype,
        //         ST_GeomFromWkb(geometry) AS geometry
        //         FROM {from}
        //         WHERE id='segment.87269954dffffff-13F6AD9C53A876A2'
        //         LIMIT {limit}"
        // ))
        .prepare(&format!(
            "SELECT
                id,
                subtype,
                ST_GeomFromWkb(geometry) AS geometry,
                road
                FROM {from}
                WHERE {where_string}
                LIMIT {limit}"
        ))
        .unwrap();
    println!("statement for transportation - end");
    #[derive(Debug)]
    struct Transportation {
        id: String,
        subtype: Option<String>,
        geom: Vec<u8>,
        road: Option<String>,
        // connectors: Option<String>,
    }
    let query_iter = stmt
        .query_map([], |row| {
            Ok(Transportation {
                id: row.get(0)?,
                subtype: row.get(1)?,
                geom: row.get(2)?,
                road: row.get(3)?,
                // connectors: row.get(2)?,
            })
        })
        .unwrap();

    println!("statement for transportation - query loaded");

    let mut bevy_transportations: Vec<BevyTransportation> = vec![];
    for item in query_iter {
        let item = item.unwrap();
        println!("statement for transportation - item road: {:?}", &item.road);
        let raw = item.geom;
        println!(
            "statement for transportation - item geom: {raw:?}:l={}",
            raw.len()
        );
        // MAGIC TO GET ARRAY THAT WORKS, COMPARED TO BINARY FROM PARQUET DIRECTLY
        // 0, 1, 104, 0, 0, 0, 0, 0, 1
        let raw = &raw[9..];
        let prefix: [u8; 2] = [1, 2];
        let raw = [prefix.as_slice(), &raw].concat();
        let mut rdr = std::io::Cursor::new(raw);
        let g = Geometry::from_wkb(&mut rdr, WkbDialect::Wkb);

        let mut base_pos: [f64; 2] = [0.; 2];
        let mut base_k: f64 = 0.;
        let mut is_base_set = false;
        match g {
            Ok(g) => match g {
                Geometry::LineString(line_string) => {
                    if !is_base_set {
                        let (k, pos) = line_string_base(&line_string);
                        base_pos = pos;
                        base_k = k;
                        is_base_set = true;
                    }
                    let bevy_transportation = line_string_road(line_string, base_k, base_pos);
                    bevy_transportations.push(bevy_transportation);
                    // dbg!(line_string);
                }
                Geometry::Polygon(polygon) => {
                    dbg!(polygon);
                    // let bevy_building =
                    //     polygon_building(polygon, base_k, base_pos, query_item.height);
                    // bevy_buildings.push(bevy_building);
                }
                not_polygon => {
                    dbg!(&not_polygon);
                }
            },
            Err(e) => {
                dbg!(e);
            }
        }
    }

    bevy_transportations
}
