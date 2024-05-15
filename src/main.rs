// on importe la "prelude" dans le namespace courant pour avoir toutes les fonctionnalités de base
// de bevy
use bevy::prelude::*;
use bevy::window::WindowResolution;

// on fait la même avec le moteur physique
use bevy_rapier2d::prelude::*;
// idem avec la crate (package rust) qui gère les animations
use bevy_spritesheet_animation::prelude::*;

// on importe nos modules (fichiers rust)
pub mod camera;
pub mod egui;
pub mod player;
pub mod world;

// on spécifie ce qu'on importe dans le namespace
use camera::CameraPlugin;
use egui::EguiDockPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

// les #[bonjour] sont des macros, celle-ci précise le point d'entrée du programme a bevy
// je sais pas si c'est utile mais la doc m'a dit de le mettre
#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Celeste-like game thing".to_string(),
                        resolution: WindowResolution::new(1280.0, 720.0),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            CameraPlugin,
            WorldPlugin,
            PlayerPlugin,
            SpritesheetAnimationPlugin,
            EguiDockPlugin,
        ))
        .run();
}

// dans bevy les fonctionnalités peuvent être bundle dans des "plugins", ça permet
// de bien séparer son architecture, ça permet aussi de très facilement intégrer des fonctionnalités
// de crates externes
//
// A noter :
// - Cargo.toml spécifie les crates rust qu'on utilise ainsi que les settings du projet rust
// - Dans rust on peut return avec le keyword return OU en omettant le ; a la fin
// exemple :
// fn valid() -> f32 {
//     32.0
// }
// fn error() -> f32 {
//     32.0;
// }
