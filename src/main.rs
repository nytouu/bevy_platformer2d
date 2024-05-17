// on importe la "prelude" dans le namespace courant pour avoir toutes les fonctionnalités de base
// de bevy
use bevy::prelude::*;
use bevy::window::WindowResolution;

// on fait la même avec le moteur physique
use bevy_rapier2d::prelude::*;
// idem avec la crate (package rust) qui gère les animations
use bevy_editor_pls::prelude::*;
// https://github.com/jakobhellermann/bevy_editor_pls
use bevy_spritesheet_animation::prelude::*;

// on importe nos modules (fichiers rust)
pub mod camera;
pub mod player;
pub mod world;

// on spécifie ce qu'on importe dans le namespace
use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

// les #[bonjour] sont des macros, celle-ci précise le point d'entrée du programme a bevy
// je sais pas si c'est utile mais la doc m'a dit de le mettre
#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            // default plugins est nécessaire pour avoir des trucs de bases de bevy
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Celeste-like game thing".to_string(),
                        resolution: WindowResolution::new(1280.0, 720.0),
                        resizable: true,
                        ..default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..Default::default()
                }),
            // physics engine
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            // animations
            SpritesheetAnimationPlugin,
            // mes plugins
            CameraPlugin,
            WorldPlugin,
            PlayerPlugin,
            EditorPlugin::default(),
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
// - la méthode .unwrap() peut se call sur tout Option<T> ou Result<T, E> afin de récupérer la valeur Some(T) ou Ok(T) si
// elle existe, si la valeur est None alors rust panic
