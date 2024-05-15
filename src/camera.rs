// pour le bloom : https://github.com/bevyengine/bevy/blob/latest/examples/2d/bloom_2d.rs

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
    // les query permettent de récupérer des components d'entités dans le World
    // c'est un générique avec 2 parties :
    // - les éléments qu'on veut query
    // - les filtres
    // doc des queries : https://bevy-cheatbook.github.io/programming/queries.html
    mut cameras: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    // on peut gérer les queries de plusieurs manières différentes :
    // - on peut itérer, safe car si elle est vide elle n'itère pas (mais ne crash pas), très
    // utile quand il y a plusieurs éléments à query
    // - on peut query.single() qui renvoie l'élément uniquement si il existe un seul élément query
    // dans le monde, panic autrement
    // - on peut query.single_mut() qui fait la même mais renvoie la/les ref mutable(s)
    // - on peut query.get_single(), pareil que single mais renvoie un Option<T>, on doit alors
    // gérer le cas ou la valeur est None, ne panic pas
    // - on peut query.get_single_mut()
    // - on peut faire comme ci dessous :
    if query.is_empty() {
        return;
    }

    // le query.single() ne peut pas panic car on return si la query est empty
    let player = query.single();
    let player_position = player.translation;

    for mut transform in &mut cameras {
        transform.translation = transform.translation.lerp(player_position, LERP_FACTOR);
    }
}
