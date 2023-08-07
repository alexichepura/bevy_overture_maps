use geozero::wkb::FromWkb;
use parquet::file::{reader::FileReader, serialized_reader::SerializedFileReader};
use std::fs::File;

#[allow(unused)]
pub fn parquet_import() {
    // let path = "theme=admins/type=locality/20230725_211237_00132_5p54t_0fa79fec-7a39-4f51-90a6-aa94b553befd";
    // let path = "theme=buildings/type=building/20230725_211555_00082_tpd52_00e93efa-24f4-4014-b65a-38c7d2854ab4";
    // let path = format!("../overture/{path}");
    let path = format!(
        "../overture/type=segment/20230726_134827_00007_dg6b6_09951677-6217-4b2f-b579-287ffed08510"
    );
    let file = File::open(path).expect("Unable to open file");

    let reader: Box<dyn FileReader> =
        Box::new(SerializedFileReader::new(file).expect("Failed to create reader"));
    let mut iter = reader.get_row_iter(None).unwrap();
    let mut c = 1;
    // let mut bevy_buildings: Vec<BevyBuilding> = vec![];
    let mut base: [f64; 2] = [0.; 2];
    let mut is_base_set = false;
    while let Some(record) = iter.next() {
        let row = record.expect("Failed to get row");
        println!("{:?}", &row);
        let mut result = Vec::new();
        for (name, field) in row.get_column_iter() {
            result.push((name, field));
            if name == "geometry" {
                if let parquet::record::Field::Bytes(bytes) = field {
                    // dbg!(bytes);
                    let raw = bytes.data();
                    // let geometry_vec = Vec::from(raw);
                    // let w = geozero::wkb::Wkb(geometry_vec);
                    // dbg!(&bytes.to_string());
                    dbg!(&bytes.to_string());
                    println!("{:?}", &raw.len());
                    let mut rdr = std::io::Cursor::new(raw);
                    let g = geo_types::Geometry::from_wkb(&mut rdr, geozero::wkb::WkbDialect::Wkb);
                    let g = g.unwrap();
                    match g {
                        geo_types::Geometry::Polygon(polygon) => {
                            if !is_base_set {
                                let c1 = polygon
                                    .exterior()
                                    .coords()
                                    .nth(0)
                                    .expect("To take exterior:0 coordinate");
                                base = [c1.x, c1.y];
                                is_base_set = true;
                            }
                            // let bevy_building = crate::building::polygon_building(polygon, base);
                            // dbg!(&bevy_building);
                            // bevy_buildings.push(bevy_building);
                        }
                        not_polygon => {
                            dbg!(&not_polygon);
                        }
                    }
                }
            }
            // dbg!(name, field);
        }
        if c >= 1 {
            break;
        }
        c += 1;
    }
}
