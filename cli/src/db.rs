use duckdb::Connection;

pub fn cache_location(lon: f64, lat: f64) {
    let shift = 0.01;
    let lat_max = lat + shift;
    let lat_min = lat - shift;
    let lon_max = lon + shift;
    let lon_min = lon - shift;

    let polygon_str = format!("{lon_min} {lat_min}, {lon_min} {lat_max}, {lon_max} {lat_max}, {lon_max} {lat_min}, {lon_min} {lat_min}", );
    let query = format!("ST_Within(ST_GeomFromWkb(geometry), ST_Envelope(ST_GeomFromText('POLYGON(({polygon_str}))')))");
    let from_string = "read_parquet('../overture/type=segment/*')".to_string();

    let path = "./data.duckdb";
    let conn = Connection::open(&path).unwrap();
    conn.execute_batch("INSTALL httpfs; LOAD httpfs;").unwrap();
    conn.execute_batch("INSTALL spatial; LOAD spatial;")
        .unwrap();

    let mut stmt = conn
        .prepare(&format!(
            "COPY (SELECT id,
                ST_GeomFromWkb(geometry) AS geometry,
                road
                FROM {from_string} WHERE {query})
            TO 'parquet/result-transportation.parquet' (FORMAT 'parquet')"
        ))
        .unwrap();

    stmt.query([]).unwrap();
}
