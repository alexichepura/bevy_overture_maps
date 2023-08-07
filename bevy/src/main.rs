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

    let shift = 0.01;
    let lat_max = lat + shift;
    let lat_min = lat - shift;
    let lon_max = lon + shift;
    let lon_min = lon - shift;

    let polygon_str = format!("{lon_min} {lat_min}, {lon_min} {lat_max}, {lon_max} {lat_max}, {lon_max} {lat_min}, {lon_min} {lat_min}", );
    let query = format!("ST_Within(ST_GeomFromWkb(geometry), ST_Envelope(ST_GeomFromText('POLYGON(({polygon_str}))')))");

    let k = geodesic_to_coord(Coord { x: lon, y: lat });
    let translate: [f64; 2] = [lon * k, lat * k];

    let bevy_transportation = query_transportation(TransportationQueryParams {
        limit: 10000,
        from_string: format!("read_parquet('parquet/{lonlatname}-transportation.parquet')"),
        where_string: query.clone(),
        k,
        translate,
    });

    let bevy_buildings = duckdb_query_buildings(BuildingsQueryParams {
        limit: 10000,
        from_string: format!("read_parquet('parquet/{lonlatname}-building.parquet')"),
        where_string: query,
        k,
        translate,
    });
    init_bevy(bevy_buildings, bevy_transportation);
}
