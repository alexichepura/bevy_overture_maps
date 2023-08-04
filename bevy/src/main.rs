use bevy::init_bevy;
use duckdb_query::{duckdb_query_buildings, BuildingsQueryParams};
use query_transportation::{query_transportation, TransportationQueryParams};

mod bevy;
mod building;
mod camera;
mod duckdb_query;
mod ground;
mod light;
mod parquet_import;
mod query_transportation;
mod transportation;

fn main() {
    // parquet_import::parquet_import();
    let lat = std::env::var("BEVY_OVERTURE_LAT").expect("BEVY_OVERTURE_LAT env");
    dbg!(&lat);
    let lat = lat.parse::<f64>().expect("lat to be f64");

    let lng = std::env::var("BEVY_OVERTURE_LNG").expect("BEVY_OVERTURE_LNG env");
    let lng = lng.parse::<f64>().expect("lng to be f64");

    let shift = 0.01;
    let lat_max = lat + shift;
    let lat_min = lat - shift;
    let lng_max = lng + shift;
    let lng_min = lng - shift;

    let polygon_str = format!("{lng_min} {lat_min}, {lng_min} {lat_max}, {lng_max} {lat_max}, {lng_max} {lat_min}, {lng_min} {lat_min}", );
    let query = format!("ST_Within(ST_GeomFromWkb(geometry), ST_Envelope(ST_GeomFromText('POLYGON(({polygon_str}))')))");

    let bevy_transportation = query_transportation(TransportationQueryParams {
        limit: 100,
        from_string: "read_parquet('../overture/type=segment/*')".to_string(),
        where_string: query.clone(),
    });

    let bevy_buildings = duckdb_query_buildings(BuildingsQueryParams {
        limit: 2,
        from_string: "read_parquet('../overture/theme=buildings/type=building/*')".to_string(),
        where_string: query,
    });
    init_bevy(bevy_buildings, bevy_transportation);
}
