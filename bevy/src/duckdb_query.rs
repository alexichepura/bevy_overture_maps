use duckdb::Connection;
use geo_types::Geometry;
use geozero::wkb::FromWkb;
use geozero::wkb::WkbDialect;

use crate::building::{polygon_building, BevyBuilding};

// https://github.com/OvertureMaps/data/issues/8 duckdb issue
// https://bertt.wordpress.com/2023/07/31/overture-maps/
// https://github.com/shi-works/Overture-Maps-Data-for-GIS // japan

pub struct BuildingsQueryParams {
    pub from_string: String,
    pub k: f64,
    pub translate: [f64; 2],
}

pub fn duckdb_query_buildings(params: BuildingsQueryParams) -> Vec<BevyBuilding> {
    let path = "./data.duckdb";
    let conn = Connection::open(&path).unwrap();
    conn.execute_batch("INSTALL httpfs; LOAD httpfs;").unwrap();
    conn.execute_batch("INSTALL spatial; LOAD spatial;")
        .unwrap();
    // let mut stmt = conn
    //     .prepare(
    //         "SELECT
    //             id,
    //             ST_GeomFromWkb(geometry) AS geometry
    //         FROM read_parquet('s3://overturemaps-us-west-2/release/2023-07-26-alpha.0/theme=buildings/type=*/*', filename=true, hive_partitioning=1)
    //         WHERE
    //                 bbox.minX > -122.4447744
    //             AND bbox.maxX < -122.2477071
    //             AND bbox.minY > 47.5621587
    //             AND bbox.maxY < 47.7120663
    //         LIMIT
    //             100",
    //     )
    //     .unwrap();
    let from = params.from_string;
    let mut stmt = conn
        .prepare(
            &format!(
                "SELECT
                id,
                height,
                JSON(names) as names,
                ST_GeomFromWkb(geometry) AS geometry
            FROM {from}"
            ),
            // WHERE
            //     bbox.minX > 139.69170 AND bbox.maxX < 139.70170 AND bbox.minY > 35.68951 AND bbox.maxY < 35.69951",
        )
        .unwrap();
    #[derive(Debug)]
    struct Bb {
        // id: String,
        height: Option<f64>,
        // names: String,
        // bbox: String,
        geom: Vec<u8>,
    }
    let query_iter = stmt
        .query_map([], |row| {
            Ok(Bb {
                // id: row.get(0)?,
                height: row.get(1)?,
                // names: row.get(2)?,
                // bbox: row.get(3)?,
                geom: row.get(3)?,
            })
        })
        .unwrap();

    let mut bevy_buildings: Vec<BevyBuilding> = vec![];
    for query_item in query_iter {
        let query_item = query_item.unwrap();
        // println!("{:?}", &query_item);
        let raw = query_item.geom;
        // MAGIC TO GET ARRAY THAT WORKS, COMPARED TO BINARY FROM PARQUET DIRECTLY
        let raw = &raw[21..]; // remove 0, 2, 96, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 5, 0, 0, 0, 0,
        let prefix: [u8; 10] = [1, 3, 0, 0, 0, 1, 0, 0, 0, 5];
        let raw = [prefix.as_slice(), raw].concat();
        let mut rdr = std::io::Cursor::new(raw);
        let g = Geometry::from_wkb(&mut rdr, WkbDialect::Wkb);
        match g {
            Ok(g) => match g {
                Geometry::Polygon(polygon) => {
                    let bevy_building =
                        polygon_building(polygon, params.k, params.translate, query_item.height);
                    bevy_buildings.push(bevy_building);
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
    bevy_buildings
}
