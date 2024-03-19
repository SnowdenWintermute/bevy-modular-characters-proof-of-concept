use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            ..Default::default()
        },
        PanOrbitCamera {
            focus: Vec3::new(0.0, 1.0, 0.0),
            radius: Some(6.0),
            ..Default::default()
        },
    ));
}
