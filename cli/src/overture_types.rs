use serde::{Deserialize, Serialize};
use std::fs::{self, File};

// https://github.com/OvertureMaps/schema/blob/main/schema/schema.yaml
pub fn schema_types() {
    // schemafy::schemafy!("../../overture_maps_schema/schema/schema.yaml");
    // schemafy::schemafy!("src/schema.json");
}

pub fn get_schema_json() {
    let path = format!("../overture_maps_schema/schema/schema.yaml");
    let file = File::open(path).expect("Unable to open file");
    // let buffered_read = std::io::BufReader::new(file);
    let json_value: serde_json::Value = serde_yaml::from_reader(file).unwrap();
    // let json_value: serde_json::Value = serde_yaml::from_str(
    //     &"
    //     ---
    //     test: some value here
    //     another: value here
    //     third:
    //         - array
    //         - another
    //     "
    //     .replace("    ", ""),
    // )
    // .unwrap();
    // serde_json::to_writer(writer, value)
    let schema_string = serde_json::to_string(&json_value).expect("Schema string");
    fs::write("./schema.json", schema_string).expect("Unable to write file");
    println!("{}", serde_json::to_string_pretty(&json_value).unwrap());
    // let f = File::create(path).expect("Unable to open file");
    // f.write_all(&1234_u32.to_be_bytes())?;
}
