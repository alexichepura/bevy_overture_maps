use bevy::init_bevy;
use duckdb_query::{duckdb_query_buildings, BuildingsQueryParams};

use geo_types::Coord;
use query_transportation::{query_transportation, TransportationQueryParams};

use crate::geo_util::geodesic_to_coord;

mod bevy;
mod building;
mod camera;
mod duckdb_query;
mod geo_util;
mod ground;
mod light;
mod parquet_import;
mod query_transportation;
mod transportation;

fn main() {
    let lat = std::env::var("MAP_LAT").expect("MAP_LAT env");
    let lat = lat.parse::<f64>().expect("lat to be f64");
    let lon = std::env::var("MAP_LON").expect("MAP_LON env");
    let lon = lon.parse::<f64>().expect("lon to be f64");
    let name = std::env::var("MAP_NAME").expect("MAP_NAME env");
    let lonlatname = format!("{lon}-{lat}-{name}");
    println!("{lonlatname}");

    let k = geodesic_to_coord(Coord { x: lon, y: lat });
    let translate: [f64; 2] = [lon * k, lat * k];

    let bevy_transportation = query_transportation(TransportationQueryParams {
        from_string: format!("read_parquet('parquet/{lonlatname}-transportation.parquet')"),
        k,
        translate,
    });

    let bevy_buildings = duckdb_query_buildings(BuildingsQueryParams {
        from_string: format!("read_parquet('parquet/{lonlatname}-building.parquet')"),
        limit: None,
        k,
        translate,
    });
    init_bevy(bevy_buildings, bevy_transportation);
}
