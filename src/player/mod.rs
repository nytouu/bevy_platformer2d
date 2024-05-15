// le nom de fichier "mod.rs" est spécial dans rust
// lorsque dans le fichier main.rs j'indique :
// mod player;
// rust cherche les fichiers ./player.rs OU ./player/mod.rs
// ça me permet de mettre plusieurs fichiers dans le dossier player tout en pouvant importer le
// "player" directement

use bevy::prelude::*;

mod dash;
use dash::Dash;

mod animation;
mod movement;
mod setup;

// pub car on veut y avoir accès dans les autres modules du player
pub const AIR_FRICTION: f32 = 50.0;
pub const JOYSTICK_THRESHOLD: f32 = 0.5;
pub const GRAVITY_SCALE: f32 = 0.8;

// la macro derive permet d'implémenter les trait indiqué
// pour les components bevy il faut utiliser cette macro
// doc : https://bevy-cheatbook.github.io/programming/ec.html#components
/// Data associée au player
#[allow(dead_code)]
#[derive(Component)]
pub struct Player {
    speed: f32,
    grounded: bool,

    jump_force: f32,
    max_jump_height: f32,

    dash_speed: f32,
    dash_max_time: f32,
    dash_reset_time: f32,
}

// le trait default permet d'instancier un objet avec des valeurs par défaut défini au compile-time
// c'est comme un "new" mais avec des valeurs par défaut
// un autre avantage d'implémenter default est de pouvoir déclarer des valeurs qui changent et de
// garder des valeurs par default :
// let player_1 = Player::default();
// let player_2 = Player {
//     speed: 50000.0,
//     jump_force: 30000.0,
//     ..Default::default(),
// }
impl Default for Player {
    fn default() -> Self {
        Player {
            speed: 20000.0,
            grounded: false,

            max_jump_height: 200.0,
            jump_force: 20000.0,

            dash_speed: 250.0,
            dash_max_time: 0.2,
            dash_reset_time: 1.0,
        }
    }
}

// les structs peuvent être des "units", des "tuples" ou des structs avec des membres
// doc : https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
// ici la le float du Jump correspond a sa hauteur
#[derive(Component)]
struct Jump(f32);

#[derive(Component, Clone)]
enum Direction {
    Right,
    Left,
}

// la macro allow permet d'autoriser certaines choses que le compilateur n'aime/n'autorise pas
// ici on enlève les warnings pour les membres non utilisés
#[allow(dead_code)]
// ici on dérive en plus de PartialEq et Eq pour pouvoir faire des comparaisons sur notre enum
#[derive(Component, PartialEq, Eq)]
enum PlayerState {
    Idle,
    Run,
    Dash,
    Jump,
    Air,
    Land,
    Climb,
    Wall,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup::setup_player).add_systems(
            Update,
            (
                // physics
                movement::check_for_ground,
                dash::dash_cooldown,
                movement::strafe,
                movement::jump,
                movement::jump_release,
                movement::rise,
                dash::dash,
                dash::dashing,
                movement::update_direction,
                movement::remove_buffer,
                movement::jump_buffer,
            )
                .chain(),
        );
        app.add_systems(
            PostUpdate,
            (
                // animation
                animation::land,
                animation::update_sprite_direction,
                animation::update_animation,
                animation::jump_to_air,
                animation::land_to_idle,
                animation::update_dash_color,
                animation::reset_dash_color,
                animation::post_dash,
                dash::spawn_dash_trail,
                dash::fade_out_trail,
            )
                .chain(),
            // on peut déclarer plusieurs systèmes dans l'update d'un coup, on peut aussi call la
            // méthode .chain() qui permet d'executer ces systèmes dans l'ordre indiqué

            // ici je n'importe pas les systèmes dans le namespace directement ça permet d'avoir
            // animation::dash et dash::dash en même temps par exemple
        );
    }
}
