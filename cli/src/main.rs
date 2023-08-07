use clap::{Args, Parser, Subcommand};

use crate::{db::cache_location, geometry::check_wkb, overture_types::get_schema_json};

mod db;
mod geometry;
mod overture_types;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CheckWkb(CheckWkbArgs),
    GetSchemaJson,
    Location(LocationArgs),
}

#[derive(Args)]
struct CheckWkbArgs {
    bytes: String,
}
#[derive(Args)]
struct LocationArgs {
    lon: String, // x
    lat: String, // y
    name: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::CheckWkb(args) => {
            let bytes_string = &args.bytes;
            let bytes_array = serde_json::from_str::<Vec<u8>>(&bytes_string).expect("bytes array");
            check_wkb(bytes_array.as_slice());
            println!("Check WKB end");
        }
        Commands::GetSchemaJson => {
            println!("GetSchemaJson start");
            get_schema_json();
            println!("GetSchemaJson end");
        }
        Commands::Location(args) => {
            println!("Location start");
            let lon = args.lon.parse::<f64>().expect("lon to be f64");
            let lat = args.lat.parse::<f64>().expect("lat to be f64");
            cache_location(lon, lat, &args.name);
            println!("Location end");
        }
    }
}
