use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraPlugin, PixelViewport, PixelZoom};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PixelCameraPlugin)
            .add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        PixelZoom::FitSize {
            width: 320,
            height: 180,
        },
        PixelViewport,
    ));
}
