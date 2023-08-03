use bevy::init_bevy;
use duckdb_query::{duckdb_query_buildings, BuildingsQueryParams};
use query_transportation::{duckdb_query_transportation, TransportationQueryParams};

mod bevy;
mod building;
mod camera;
mod duckdb_query;
mod query_transportation;
mod transportation;
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
    let bevy_transportation = duckdb_query_transportation(TransportationQueryParams { 
        limit: 1, 
        from_string: "read_parquet('./transportation-connector-japan.parquet')".to_string(),
        where_string: "ST_Within(ST_GeomFromWkb(geometry), ST_Envelope(ST_GeomFromText('POLYGON((139.68170 35.67951, 139.68170 35.69951, 139.70170 35.69951, 139.70170 35.67951, 139.68170 35.67951))')))".to_string() 
    });
    init_bevy(bevy_buildings, bevy_transportation);
}
