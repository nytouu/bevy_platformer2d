use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::*;
use super::Direction;

#[derive(Component)]
pub struct Dash {
    pub elapsed: f32,
    pub direction: DashDirection,
}

impl Dash {
    pub fn new(direction: DashDirection) -> Dash {
        Dash {
            elapsed: 0.0,
            direction,
        }
    }
}

pub enum DashDirection {
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl DashDirection {
    pub fn get_direction(&self) -> Option<Direction> {
        match self {
            // keep same direction
            DashDirection::North | DashDirection::South => None,

            // handle direction change
            DashDirection::West | DashDirection::NorthWest | DashDirection::SouthWest => {
                Some(Direction::Left)
            }
            DashDirection::East | DashDirection::NorthEast | DashDirection::SouthEast => {
                Some(Direction::Right)
            }
        }
    }
}

pub fn dash(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    // mouse: Res<ButtonInput<MouseButton>>,
    query: Query<Entity, (Without<Dash>, With<Player>)>,
) {
    if query.is_empty() {
        return;
    }

    let entity = query.single();

    let mut dash: bool = false;

    if input.any_pressed([KeyCode::ShiftLeft, KeyCode::Enter]) {
        dash = true;
    } else {
        for gamepad in gamepads.iter() {
            if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::West)) {
                dash = true;
            };
        }
    }

    // check direction
    if dash {
        let mut direction = Vec2::ZERO;

        // kb
        // input.get_pressed() renvoie un interator, on doit utiliser into_iter() pour pouvoir
        // utiliser le keyword for 
        for key in input.get_pressed().into_iter() {
            match key {
                KeyCode::KeyW | KeyCode::ArrowUp => direction += Vec2::new(0.0, 1.0),
                KeyCode::KeyS | KeyCode::ArrowDown => direction += Vec2::new(0.0, -1.0),
                KeyCode::KeyA | KeyCode::ArrowLeft => direction += Vec2::new(-1.0, 0.0),
                KeyCode::KeyD | KeyCode::ArrowRight => direction += Vec2::new(1.0, 0.0),
                _ => {}
            }
        }

        // gamepad
        for gamepad in gamepads.iter() {
            // dpad
            if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadUp)) {
                direction += Vec2::new(0.0, 1.0);
            }
            if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadDown)) {
                direction += Vec2::new(0.0, -1.0);
            }
            if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft)) {
                direction += Vec2::new(-1.0, 0.0);
            }
            if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadRight)) {
                direction += Vec2::new(1.0, 0.0);
            }

            // joystick
            let left_stick_x = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .unwrap();
            if left_stick_x.abs() > JOYSTICK_THRESHOLD {
                direction.x += if left_stick_x > 0.0 { 1.0 } else { -1.0 };
            }
            let left_stick_y = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
                .unwrap();
            if left_stick_y.abs() > JOYSTICK_THRESHOLD {
                direction.y += if left_stick_y > 0.0 { 1.0 } else { -1.0 };
            }
        }

        // TODO: faire en sorte que ça soit pas un match dégueu comme ça
        let dash_direction: Option<DashDirection>;
        match direction {
            Vec2 { x, y } if x > 0.0 && y == 0.0 => {
                dash_direction = Some(DashDirection::East);
            }
            Vec2 { x, y } if x < 0.0 && y == 0.0 => {
                dash_direction = Some(DashDirection::West);
            }
            Vec2 { x, y } if x == 0.0 && y > 0.0 => {
                dash_direction = Some(DashDirection::North);
            }
            Vec2 { x, y } if x == 0.0 && y < 0.0 => {
                dash_direction = Some(DashDirection::South);
            }
            Vec2 { x, y } if x > 0.0 && y > 0.0 => {
                dash_direction = Some(DashDirection::NorthEast);
            }
            Vec2 { x, y } if x > 0.0 && y < 0.0 => {
                dash_direction = Some(DashDirection::SouthEast);
            }
            Vec2 { x, y } if x < 0.0 && y > 0.0 => {
                dash_direction = Some(DashDirection::NorthWest);
            }
            Vec2 { x, y } if x < 0.0 && y < 0.0 => {
                dash_direction = Some(DashDirection::SouthWest);
            }
            _ => dash_direction = None,
        }

        if dash_direction.is_some() {
            let direction = dash_direction.unwrap();

            if direction.get_direction().is_some() {
                commands
                    .entity(entity)
                    .insert(direction.get_direction().unwrap());
            }
            info!("dash");
            commands
                .entity(entity)
                .remove::<Jump>()
                .remove::<GravityScale>()
                .insert(PlayerState::Dash)
                .insert(Dash::new(direction));
        }
    }
}

pub fn dashing(
    mut query: Query<(Entity, &mut Velocity, &Player, &mut Dash)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut velocity, player, mut dash) = query.single_mut();

    let movement = match dash.direction {
        DashDirection::North => Vec2::new(0.0, 1.0),
        DashDirection::South => Vec2::new(0.0, -1.0),
        DashDirection::West => Vec2::new(-1.0, 0.0),
        DashDirection::East => Vec2::new(1.0, 0.0),
        DashDirection::NorthWest => Vec2::new(-1.0, 1.0).normalize(),
        DashDirection::NorthEast => Vec2::new(1.0, 1.0).normalize(),
        DashDirection::SouthWest => Vec2::new(-1.0, -1.0).normalize(),
        DashDirection::SouthEast => Vec2::new(1.0, -1.0).normalize(),
    };

    if dash.elapsed > player.dash_max_time {
        commands
            .entity(entity)
            .remove::<Dash>()
            .insert(GravityScale(GRAVITY_SCALE));
    } else {
        dash.elapsed += time.delta_seconds();
        velocity.linvel = movement * player.dash_speed;
    }
}
