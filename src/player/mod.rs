use bevy::prelude::*;

mod animation;
mod movement;
mod setup;

use animation::*;
use movement::*;
use setup::*;

#[derive(Component)]
pub struct Player {
    speed: f32,
    max_jump_height: f32,
    jump_force: f32,
}

#[derive(Component)]
struct Jump(f32);

#[derive(Component)]
enum Direction {
    Right,
    Left,
}

#[allow(dead_code)]
#[derive(Component)]
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
        app.add_systems(Startup, setup_player)
            .add_systems(
                Update,
                (
                    strafe,
                    jump,
                    rise,
                    fall,
                    land,
                    update_direction,
                    update_sprite_direction,
                    update_animation,
                    jump_to_air,
                    land_to_idle,
                )
                    .chain(),
            );
    }
}
