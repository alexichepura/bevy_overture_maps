# bevy_overture_maps

Bevy overture is an example of integration between <https://overturemaps.org> and <https://bevyengine.org>

![v0.3.2 screenshot](https://github.com/alexichepura/bevy_overture_maps/assets/5582266/bb357732-953a-4b79-b759-ec26b343dff5)

## Howto

```sh
uvx overturemaps download --bbox=-71.068,42.353,-71.058,42.363 \
    -f geoparquet --type=building -o parquet/-71.068_42.353_boston_building.parquet
uvx overturemaps download --bbox=-71.068,42.353,-71.058,42.363 \
    -f geoparquet --type=segment -o parquet/-71.068_42.353_boston_transportation.parquet
```

To generate location - download overture maps data. Full size ~200G.\
<https://github.com/OvertureMaps/data>\
<https://docs.overturemaps.org/getting-data/cloud-sources>

Example command:

```sh
aws s3 cp --no-sign-request --recursive s3://overturemaps-us-west-2/release/2026-04-15.0/ <DESTINATION>
```

Only segments

```sh
aws s3 cp --no-sign-request --recursive s3://overturemaps-us-west-2/release/2026-04-15.0/theme=transportation/type=segment/ ./theme=transportation/type=segment/
```

Only buildings

```sh
aws s3 cp --no-sign-request --recursive s3://overturemaps-us-west-2/release/2026-04-15.0/theme=buildings/ ./theme=buildings/
```

Example location
`bevy_overture_maps_cli location <LON> <LAT> <NAME>`

```sh
cargo cli location 139.69170 35.68951 tokyo
```

!!! Fill `.env`, see `.env.example` !!!

Run bevy.

```sh
cargo run --release -p=bevy_overture_maps_app
 # or
cargo app
```

## Old

One coordinate flipped and lots of bugs but it works.

https://github.com/alexichepura/bevy_overture/assets/5582266/14a074e3-d520-4035-b78f-4d42ea2872ce

## License

This project is licensed under the terms of the
[MIT license](/LICENSE-MIT).
