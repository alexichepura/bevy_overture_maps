use duckdb::Connection;
use geo_types::Geometry;
use geozero::wkb::FromWkb;
use geozero::wkb::WkbDialect;

use crate::building::{polygon_building, Building};
use crate::BuildingClass;

// https://github.com/OvertureMaps/data/issues/8 duckdb issue
// https://bertt.wordpress.com/2023/07/31/overture-maps/
// https://github.com/shi-works/Overture-Maps-Data-for-GIS // japan

pub struct BuildingsQueryParams {
    pub from_string: String,
    pub limit: Option<u32>,
    pub k: f64,
    pub center: [f64; 2],
}

pub fn query_buildings(params: BuildingsQueryParams) -> Vec<Building> {
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
            "SELECT id,
                height,
                JSON(names) as names,
                geometry,
                numFloors,
                class,
            FROM {from} {limit}"
        ))
        .unwrap();
    #[derive(Debug)]
    struct DbBuilding {
        id: String,
        height: Option<f64>,
        // names: String,
        // bbox: String,
        geom: Vec<u8>,
        num_floors: Option<i32>,
        class: Option<String>,
    }
    let query_iter = stmt
        .query_map([], |row| {
            Ok(DbBuilding {
                id: row.get(0)?,
                height: row.get(1)?,
                // names: row.get(2)?,
                // bbox: row.get(3)?,
                geom: row.get(3)?,
                num_floors: row.get(4)?,
                class: row.get(5)?,
            })
        })
        .unwrap();

    let mut buildings: Vec<Building> = vec![];
    for query_item in query_iter {
        let query_item = query_item.unwrap();
        let id = query_item.id;
        let raw = query_item.geom;
        // println!("query_item.geom:{:?}", &raw);
        let mut rdr = std::io::Cursor::new(raw);
        let g = Geometry::from_wkb(&mut rdr, WkbDialect::Wkb);

        let building_class = query_item.class.map(|c| BuildingClass::from_string(&c));
        match g {
            Ok(g) => match g {
                Geometry::MultiPolygon(multy_polygon) => {
                    for polygon in multy_polygon {
                        let exterior = polygon.exterior();
                        let c1 = exterior
                            .coords()
                            .nth(0)
                            .expect("To take exterior:0 coordinate");
                        for (i, c) in exterior.coords().enumerate() {
                            if i > 0 {
                                let dlat = c.x - c1.x;
                                if dlat > 0.1 {
                                    println!("{id}:{i}dlat:{dlat}:{:?}", &polygon);
                                }
                                let dlon = c.y - c1.y;
                                if dlon > 0.1 {
                                    println!("{id}:{i}dlon:{dlon}:{:?}", &polygon);
                                }
                            }
                        }

                        let building = polygon_building(
                            polygon,
                            params.k,
                            params.center,
                            query_item.height,
                            query_item.num_floors,
                        );

                        let building = Building::from_props(building, building_class);
                        buildings.push(building);
                    }
                }
                Geometry::Polygon(polygon) => {
                    let exterior = polygon.exterior();
                    let c1 = exterior
                        .coords()
                        .nth(0)
                        .expect("To take exterior:0 coordinate");
                    for (i, c) in exterior.coords().enumerate() {
                        if i > 0 {
                            let dlat = c.x - c1.x;
                            if dlat > 0.1 {
                                println!("{id}:{i}dlat:{dlat}:{:?}", &polygon);
                            }
                            let dlon = c.y - c1.y;
                            if dlon > 0.1 {
                                println!("{id}:{i}dlon:{dlon}:{:?}", &polygon);
                            }
                        }
                    }

                    let building = polygon_building(
                        polygon,
                        params.k,
                        params.center,
                        query_item.height,
                        query_item.num_floors,
                    );
                    let building = Building::from_props(building, building_class);
                    buildings.push(building);
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
    buildings
}
