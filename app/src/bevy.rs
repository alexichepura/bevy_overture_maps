use bevy::{pbr::DirectionalLightShadowMap, prelude::*, window::WindowResolution};
use bevy_overture_maps::{
    buildings_start, transportations_start, Building, Buildings, MapMaterialHandle, Segment,
    SegmentsRes,
};

use crate::{
    camera::PlayerCameraPlugin,
    config::SceneConfig,
    ground::plane_start,
    light::{animate_light_direction, light_start_system},
};

pub fn init_bevy(buildings: Vec<Building>, segments: Vec<Segment>) {
    let mut app = App::new();

    #[cfg(not(target_arch = "wasm32"))]
    let res = WindowResolution::default();
    #[cfg(target_arch = "wasm32")]
    let res = WindowResolution::new(720., 360.);

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Overture Maps".to_string(),
                resolution: res,
                canvas: Some("#bevy-overture-maps".to_string()),
                ..default()
            }),
            ..default()
        }),
        PlayerCameraPlugin,
        #[cfg(feature = "fps")]
        crate::dash::DashPlugin,
    ))
    .init_resource::<MapMaterialHandle>()
    .insert_resource(Msaa::Sample4)
    .insert_resource(DirectionalLightShadowMap { size: 2048 * 2 })
    .insert_resource(SceneConfig::default())
    .insert_resource(Buildings { buildings })
    .insert_resource(SegmentsRes { segments })
    .add_systems(
        Startup,
        (
            plane_start,
            light_start_system,
            buildings_start,
            transportations_start,
        ),
    )
    .add_systems(Update, (animate_light_direction,));

    app.run();
}
