use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraPlugin, PixelViewport, PixelZoom};

use bevy::core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping};

use super::player::Player;
use super::egui::MainCamera;

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
        MainCamera,
        // PickRaycastSource,
    ));
}

fn camera_follow(
    query: Query<&Transform, With<Player>>,
    mut cameras: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if query.is_empty() {
        return;
    }

    let player = query.single();
    let player_position = player.translation;

    for mut transform in &mut cameras {
        transform.translation = transform.translation.lerp(player_position, LERP_FACTOR);
    }
}
