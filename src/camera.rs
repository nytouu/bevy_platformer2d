use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraPlugin, PixelViewport, PixelZoom};

use bevy::core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping};

use super::player::Player;

const LERP_FACTOR: f32 = 0.06;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PixelCameraPlugin)
            .add_systems(Startup, setup_camera)
            .add_systems(PostUpdate, camera_follow);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::AcesFitted,
            ..default()
        },
        BloomSettings::default(),
        PixelZoom::FitSize {
            width: 320,
            height: 180,
        },
        PixelViewport,
    ));
}

fn camera_follow(
    query: Query<&Transform, With<Player>>,
    mut cameras: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if query.is_empty() {
        return;
    }

    let transform = query.single();
    let pos = transform.translation;

    for mut transform in &mut cameras {
        transform.translation = transform.translation.lerp(pos, LERP_FACTOR);
    }
}
