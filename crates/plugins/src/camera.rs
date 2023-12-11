use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 4.5;

pub struct CameraPlugin;

impl Plugin for CameraPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, spawn_camera);
    }
}
 
fn spawn_camera(mut commands: Commands)
{
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, CAMERA_DISTANCE, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
