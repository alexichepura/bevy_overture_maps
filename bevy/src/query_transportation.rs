use duckdb::Connection;
use geo_types::Geometry;
use geozero::wkb::FromWkb;
use geozero::wkb::WkbDialect;

use crate::transportation::BevyTransportation;

pub struct TransportationQueryParams {
    pub limit: usize,
    pub where_string: String,
    pub from_string: String,
}

pub fn duckdb_query_transportation(params: TransportationQueryParams) -> Vec<BevyTransportation> {
    let path = "./data.duckdb";
    let conn = Connection::open(&path).unwrap();
    conn.execute_batch("INSTALL httpfs; LOAD httpfs;").unwrap();
    conn.execute_batch("INSTALL spatial; LOAD spatial;")
        .unwrap();
    let limit = params.limit;
    let where_string = params.where_string;
    let from = params.from_string;
    println!("statement for building - start");
    let mut stmt = conn
        .prepare(&format!(
            "SELECT
                id,
                subtype,
                ST_GeomFromWkb(geometry) AS geometry
                FROM {from}
                WHERE {where_string}
                LIMIT {limit}"
        ))
        // connectors,
        // road,
        // "SELECT
        //     id,
        //     updatetime,
        //     version,
        //     level,
        //     subtype,
        //     connectors,
        //     road,
        //     sources,
        // FROM {from}
        // WHERE {where_string}
        // LIMIT {limit}"
        .unwrap();
    println!("statement for building - end");
    #[derive(Debug)]
    struct Transportation {
        id: String,
        subtype: Option<String>,
        geom: Vec<u8>,
        // connectors: Option<String>,
        // road: Option<String>,
    }
    let query_iter = stmt
        .query_map([], |row| {
            Ok(Transportation {
                id: row.get(0)?,
                subtype: row.get(1)?,
                geom: row.get(2)?,
                // connectors: row.get(2)?,
                // road: row.get(3)?,
            })
        })
        .unwrap();

    println!("statement for building - query loaded");

    for item in query_iter {
        let item = item.unwrap();
        // dbg!(&item);
        let raw = item.geom;
        // MAGIC TO GET ARRAY THAT WORKS, COMPARED TO BINARY FROM PARQUET DIRECTLY
        let raw = &raw[21..]; // remove 0, 2, 96, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 5, 0, 0, 0, 0,
        let prefix: [u8; 10] = [1, 3, 0, 0, 0, 1, 0, 0, 0, 5];
        let raw = [prefix.as_slice(), raw].concat();
        let mut rdr = std::io::Cursor::new(raw);
        let g = Geometry::from_wkb(&mut rdr, WkbDialect::Wkb);
        match g {
            Ok(g) => match g {
                Geometry::Polygon(polygon) => {
                    dbg!(polygon);
                    // if !is_base_set {
                    //     let (k, pos) = polygon_base(&polygon);
                    //     base_pos = pos;
                    //     base_k = k;
                    //     is_base_set = true;
                    // }
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

    let mut bevy_transportations: Vec<BevyTransportation> = vec![];
    bevy_transportations
}
