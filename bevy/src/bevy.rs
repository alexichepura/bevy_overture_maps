use bevy::{pbr::DirectionalLightShadowMap, prelude::*, window::WindowResolution};

use crate::{
    building::{buildings_start, BevyBuilding, BevyBuildings},
    camera::PlayerCameraPlugin,
    ground::plane_start,
    light::{animate_light_direction, light_start_system},
    transportation::{transportations_start, BevyTransportation, BevyTransportations},
};

pub fn init_bevy(buildings: Vec<BevyBuilding>, transportations: Vec<BevyTransportation>) {
    let mut app = App::new();

    #[cfg(not(target_arch = "wasm32"))]
    let res = WindowResolution::default();
    #[cfg(target_arch = "wasm32")]
    let res = WindowResolution::new(720., 360.);

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Overture".to_string(),
                resolution: res,
                canvas: Some("#bevy-overture".to_string()),
                ..default()
            }),
            ..default()
        }),
        PlayerCameraPlugin,
    ))
    .insert_resource(Msaa::Sample4)
    .insert_resource(DirectionalLightShadowMap::default())
    .insert_resource(BevyBuildings { buildings })
    .insert_resource(BevyTransportations { transportations })
    .add_systems(
        Startup,
        (
            plane_start,
            light_start_system,
            buildings_start,
            transportations_start,
        ),
    )
    .add_systems(Update, animate_light_direction);

    app.run();
}
