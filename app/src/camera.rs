use bevy::{camera_controller::free_camera::FreeCamera, prelude::*};

// https://github.com/bevyengine/bevy/blob/latest/examples/camera/free_camera_controller.rs

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 0.0).looking_to(Vec3::X, Vec3::Y),
        FreeCamera {
            sensitivity: 0.2,
            friction: 25.0,
            walk_speed: 10.0,
            run_speed: 50.0,
            ..default()
        },
    ));
}
