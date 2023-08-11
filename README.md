# bevy_overture_maps
Bevy overture is an example of integration between <https://overturemaps.org> and <https://bevyengine.org/>

![v0.3 screenshot](https://github.com/alexichepura/bevy_overture_maps/assets/5582266/79513350-b7b5-4324-b955-13f5c25063f4)

https://github.com/alexichepura/bevy_overture/assets/5582266/14a074e3-d520-4035-b78f-4d42ea2872ce

## Howto
To generate location - download overture maps data. Full size ~200G.
https://github.com/OvertureMaps/data
Example command: `aws s3 cp --region us-west-2 --no-sign-request --recursive s3://overturemaps-us-west-2/release/2023-07-26-alpha.0/ <DESTINATION>`
Only segments `aws s3 cp --region us-west-2 --no-sign-request --recursive s3://overturemaps-us-west-2/release/2023-07-26-alpha.0/theme=transportation/type=segment/ ./theme=transportation/type=segment/`
Only buildings `aws s3 cp --region us-west-2 --no-sign-request --recursive s3://overturemaps-us-west-2/release/2023-07-26-alpha.0/theme=buildings/ ./theme=buildings/`

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

## License

This project is licensed under the terms of the
[MIT license](/LICENSE-MIT).
