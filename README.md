# bevy_overture 
Bevy overture is an example of integration between <https://overturemaps.org> and <https://bevyengine.org/>



https://github.com/alexichepura/bevy_overture/assets/5582266/14a074e3-d520-4035-b78f-4d42ea2872ce

## Howto
https://github.com/OvertureMaps/data
Example command: `aws s3 cp --region us-west-2 --no-sign-request --recursive s3://overturemaps-us-west-2/release/2023-07-26-alpha.0/ <DESTINATION>`

Example location
`bevy_overture_cli location <LON> <LAT> <NAME>`
```sh
cargo cli location 139.69170 35.68951 tokyo
```

!!! Fill `.env`, see `.env.example` !!!

Run bevy. `cargo run --release -p=bevy_overture`
```sh
cargo run --release -p=bevy_overture
 # or
cargo bevy
```

## License

This project is licensed under the terms of the
[MIT license](/LICENSE-MIT).
