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
    let mut stmt = conn
        .prepare(
            &format!(
                "SELECT
                id,
                height,
                JSON(names) as names,
                ST_GeomFromWkb(geometry) AS geometry
            FROM {from}
            WHERE {where_string}
            LIMIT {limit}"
            ),
            // WHERE
            //     bbox.minX > 139.69170 AND bbox.maxX < 139.70170 AND bbox.minY > 35.68951 AND bbox.maxY < 35.69951",
        )
        .unwrap();
    println!("success SELECT");
    #[derive(Debug)]
    struct Bb {
        id: String,
        // height: Option<f64>,
        // names: String,
        // bbox: String,
        // geom: Vec<u8>,
    }
    let query_iter = stmt
        .query_map([], |row| {
            Ok(Bb {
                id: row.get(0)?,
                // height: row.get(1)?,
                // names: row.get(2)?,
                // bbox: row.get(3)?,
                // geom: row.get(3)?,
            })
        })
        .unwrap();
    let mut bevy_transportations: Vec<BevyTransportation> = vec![];
    bevy_transportations
}
