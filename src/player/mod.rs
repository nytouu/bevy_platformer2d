use bevy::prelude::*;

mod animation;
mod movement;
mod setup;

#[derive(Component)]
pub struct Player {
    speed: f32,
    max_jump_height: f32,
    max_dash_length: f32,
    jump_force: f32,
    dash_speed: f32,
}

#[derive(Component)]
struct Jump(f32);

#[derive(Component)]
enum Direction {
    Right,
    Left,
}

enum DashDirection {
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

#[derive(Component)]
struct Dash {
    pub initial_position: Vec2,
    pub value: f32,
    pub direction: DashDirection,
}

impl DashDirection {
    fn get_direction(&self) -> Option<Direction> {
        match self {
            // keep same direction
            DashDirection::North | DashDirection::South => None,

            // handle direction change
            DashDirection::West | DashDirection::NorthWest | DashDirection::SouthWest => Some(Direction::Left),
            DashDirection::East | DashDirection::NorthEast | DashDirection::SouthEast => Some(Direction::Right),
        }
    }
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
        app.add_systems(Startup, setup::setup_player).add_systems(
            Update,
            (
                // physics
                movement::strafe,
                movement::jump,
                movement::jump_release,
                movement::rise,
                movement::fall,
                movement::dash,
                movement::dashing,
                movement::update_direction,

                // animation
                animation::land,
                animation::update_sprite_direction,
                animation::update_animation,
                animation::jump_to_air,
                animation::land_to_idle,
                animation::update_dash_color,
                animation::reset_dash_color,
                animation::post_dash,
            )
                .chain(),
        );
    }
}
