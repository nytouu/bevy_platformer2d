// on importe la "prelude" dans le namespace courant pour avoir toutes les fonctionnalités de base
// de bevy
use bevy::prelude::*;

// on fait la même avec le moteur physique
use bevy_rapier2d::prelude::*;
// idem avec la crate (package rust) qui gère les animations
use bevy_spritesheet_animation::prelude::*;

// on importe nos modules (fichiers rust)
mod camera;
mod player;
mod world;
mod config;
mod editor;
mod tile;

// on spécifie ce qu'on importe dans le namespace
use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;
use config::ConfigPlugin;
use editor::PlsEditorPlugin;
use tile::TileWorldPlugin;

// les #[bonjour] sont des macros, celle-ci précise le point d'entrée du programme a bevy
// je sais pas si c'est utile mais la doc m'a dit de le mettre
#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            // default plugins est nécessaire pour avoir des trucs de bases de bevy
            ConfigPlugin,
            // physics engine
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            // animations
            SpritesheetAnimationPlugin,
            // mes plugins
            CameraPlugin,
            WorldPlugin,
            TileWorldPlugin,
            PlayerPlugin,
            PlsEditorPlugin,
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
