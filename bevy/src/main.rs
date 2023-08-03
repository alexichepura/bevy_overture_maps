use bevy::init_bevy;
use duckdb_query::{duckdb_query_buildings, BuildingsQueryParams};

mod bevy;
mod building;
mod camera;
mod duckdb_query;
mod ground;
mod light;
mod parquet_import;

fn main() {
    // https://docs.overturemaps.org/
    // https://www.ogc.org/standard/sfa/
    let bevy_buildings = duckdb_query_buildings(BuildingsQueryParams { 
        limit: 1000, 
        from_string: "read_parquet('./buildings-japan.parquet')".to_string(),
        where_string: "ST_Within(ST_GeomFromWkb(geometry), ST_Envelope(ST_GeomFromText('POLYGON((139.68170 35.67951, 139.68170 35.69951, 139.70170 35.69951, 139.70170 35.67951, 139.68170 35.67951))')))".to_string() 
    });
    init_bevy(bevy_buildings);
}
