use bevy::{pbr::NotShadowCaster, prelude::*};

use crate::config::SceneConfig;

pub fn plane_start(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scene_config: Res<SceneConfig>,
) {
    let size = scene_config.size;
    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(size).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        NotShadowCaster,
    ));

    // cmd.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..default()
    // });

    // cmd.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(0., 10., 20.).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
}
