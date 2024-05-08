use bevy::prelude::*;

mod movement;
mod setup;

use movement::*;
use setup::*;

#[derive(Component)]
struct Player {
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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player).add_systems(
            Update,
            (
                strafe,
                jump,
                rise,
                fall,
                update_direction,
                update_sprite_direction,
            )
                .chain(),
        );
    }
}
