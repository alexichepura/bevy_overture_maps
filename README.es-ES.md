

# bevy_overture_maps

Bevy overture es un ejemplo de integración entre <https://overturemaps.org> y <https://bevyengine.org>

![v0.3.2 screenshot](https://github.com/alexichepura/bevy_overture_maps/assets/5582266/bb357732-953a-4b79-b759-ec26b343dff5)

## Cómo usarlo

### Usando `overturemaps download` (Recomendado)
```sh
uvx overturemaps download --bbox=-71.068,42.353,-71.058,42.363 \
    -f geoparquet --type=building -o parquet/-71.068_42.353_boston_building.parquet
uvx overturemaps download --bbox=-71.068,42.353,-71.058,42.363 \
    -f geoparquet --type=segment -o parquet/-71.068_42.353_boston_transportation.parquet
```
Configuración relevante de `.env`
```sh
MAP_NAME="boston"
MAP_LON="-71.068"
MAP_LAT="42.353"
```

### Usando todos los datos y extrayéndolos localmente

Para generar la ubicación, descarga los datos de Overture Maps. Tamaño total ~200G.\
<https://github.com/OvertureMaps/data>\
<https://docs.overturemaps.org/getting-data/cloud-sources>

Ejemplo de comando:

```sh
aws s3 cp --no-sign-request --recursive s3://overturemaps-us-west-2/release/2026-04-15.0/ <DESTINATION>
```

Solo segmentos

```sh
aws s3 cp --no-sign-request --recursive s3://overturemaps-us-west-2/release/2026-04-15.0/theme=transportation/type=segment/ ./theme=transportation/type=segment/
```

Solo edificios

```sh
aws s3 cp --no-sign-request --recursive s3://overturemaps-us-west-2/release/2026-04-15.0/theme=buildings/ ./theme=buildings/
```

Ejemplo de ubicación
`bevy_overture_maps_cli location <LON> <LAT> <NAME>`

```sh
cargo cli location 139.69170 35.68951 tokyo
```

!!! Rellena el `.env`, consulta `.env.example` !!!

Ejecuta Bevy.

```sh
cargo run --release -p=bevy_overture_maps_app
 # or
cargo app
```

## Anterior

Una coordenada está invertida y hay muchos errores, pero funciona.

https://github.com/alexichepura/bevy_overture/assets/5582266/14a074e3-d520-4035-b78f-4d42ea2872ce

## Licencia

Este proyecto está licenciado bajo los términos de la
[MIT license](/LICENSE-MIT).
