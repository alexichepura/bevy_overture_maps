use bevy::{
    light::{light_consts::lux, CascadeShadowConfigBuilder, FogVolume, VolumetricLight},
    prelude::*,
};

// https://github.com/bevyengine/bevy/blob/latest/examples/3d/atmosphere.rs

use crate::config::SceneConfig;

pub fn light_start_system(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scene_config: Res<SceneConfig>,
) {
    // Configure a properly scaled cascade shadow map for this scene (defaults are too large, mesh units are in km)
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 1000.0,
        ..default()
    }
    .build();

    // Sun
    cmd.spawn((
        DirectionalLight {
            shadows_enabled: true,
            // lux::RAW_SUNLIGHT is recommended for use with this feature, since
            // other values approximate sunlight *post-scattering* in various
            // conditions. RAW_SUNLIGHT in comparison is the illuminance of the
            // sun unfiltered by the atmosphere, so it is the proper input for
            // sunlight to be filtered by the atmosphere.
            illuminance: lux::RAW_SUNLIGHT,
            ..default()
        },
        Transform::from_xyz(1.0, 0.4, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        VolumetricLight,
        cascade_shadow_config,
    ));

    // spawn the fog volume
    cmd.spawn((
        FogVolume::default(),
        Transform::from_scale(Vec3::new(10.0, 1.0, 10.0)).with_translation(Vec3::Y * 0.5),
    ));

    // cmd.spawn((
    //     DirectionalLight {
    //         illuminance: 40_000.,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     Transform {
    //         translation: Vec3::new(0., 0., 0.),
    //         rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_8),
    //         ..default()
    //     },
    // ));

    // cmd.spawn((
    //     Mesh3d(meshes.add(Cuboid::default().mesh())),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::srgb(0.53, 0.53, 0.53),
    //         unlit: true,
    //         cull_mode: None,
    //         ..default()
    //     })),
    //     Transform::from_scale(Vec3::splat(scene_config.size)),
    // ));
}

const K: f32 = 2.;

pub fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.pressed(KeyCode::KeyH) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_secs() * K);
        }
    }
    if input.pressed(KeyCode::KeyL) {
        for mut transform in &mut query {
            transform.rotate_y(-time.delta_secs() * K);
        }
    }
    if input.pressed(KeyCode::KeyJ) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_secs() * K);
        }
    }
    if input.pressed(KeyCode::KeyK) {
        for mut transform in &mut query {
            transform.rotate_x(-time.delta_secs() * K);
        }
    }
}
