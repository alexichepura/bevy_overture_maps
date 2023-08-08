use bevy::init_bevy;
use bevy_overture_maps::{
    geodesic_to_coord, query_buildings, query_transportation, BuildingsQueryParams,
    TransportationQueryParams,
};
use geo_types::Coord;

mod bevy;
mod camera;
mod ground;
mod light;
mod parquet_import;

fn main() {
    let lat = std::env::var("MAP_LAT").expect("MAP_LAT env");
    let lat = lat.parse::<f64>().expect("lat to be f64");
    let lon = std::env::var("MAP_LON").expect("MAP_LON env");
    let lon = lon.parse::<f64>().expect("lon to be f64");
    let name = std::env::var("MAP_NAME").expect("MAP_NAME env");
    let lonlatname = format!("{lon}_{lat}_{name}");
    println!("{lonlatname}");

    let k = geodesic_to_coord(Coord { x: lon, y: lat });
    let center_xz: [f64; 2] = [lon * k, -lat * k]; // Yto-Z

    let from_transportation =
        format!("read_parquet('parquet/{lonlatname}_transportation.parquet')");
    let from_building = format!("read_parquet('parquet/{lonlatname}_building.parquet')");
    println!("from_transportation:{}", &from_transportation);
    println!("from_building:{}", &from_building);

    let bevy_transportation = query_transportation(TransportationQueryParams {
        from_string: from_transportation,
        limit: None,
        k,
        center: center_xz,
    });

    let bevy_buildings = query_buildings(BuildingsQueryParams {
        from_string: from_building,
        limit: None,
        k,
        center: center_xz,
    });
    init_bevy(bevy_buildings, bevy_transportation);
}
