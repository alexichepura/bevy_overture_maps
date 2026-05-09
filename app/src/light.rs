use bevy::prelude::*;

use crate::config::SceneConfig;

pub fn light_start_system(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scene_config: Res<SceneConfig>,
) {
    cmd.spawn((
        DirectionalLight {
            illuminance: 40_000.,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 0., 0.),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_8),
            ..default()
        },
    ));

    cmd.spawn((
        Mesh3d(meshes.add(Cuboid::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.53, 0.53, 0.53),
            unlit: true,
            cull_mode: None,
            ..default()
        })),
        Transform::from_scale(Vec3::splat(scene_config.size)),
    ));
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
