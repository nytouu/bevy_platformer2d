use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use super::Direction;
use super::*;

const JOYSTICK_THRESHOLD: f32 = 0.5;

pub fn strafe(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut KinematicCharacterController,
            &KinematicCharacterControllerOutput,
            &Player,
            &SpritesheetAnimation,
        ),
        Without<Dash>,
    >,
    library: Res<SpritesheetLibrary>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut controller, output, player, animation) = query.single_mut();

    let mut movement = Vec2::new(0.0, 0.0);

    if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        movement.x += time.delta_seconds() * player.speed;
    }

    if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        movement.x += time.delta_seconds() * player.speed * -1.0;
    }

    if movement == Vec2::ZERO {
        for gamepad in gamepads.iter() {
            let left_stick_x = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .unwrap();
            if left_stick_x.abs() > JOYSTICK_THRESHOLD {
                movement.x += time.delta_seconds() * player.speed * left_stick_x;
            }
            // Dpad
            if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadRight)) {
                movement.x += time.delta_seconds() * player.speed;
            }
            if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft)) {
                movement.x += time.delta_seconds() * player.speed * -1.0;
            }
        }
    }

    if output.grounded {
        let land = library.animation_with_name("player_land").unwrap();

        if movement == Vec2::ZERO {
            if animation.animation_id != land {
                commands.entity(entity).insert(PlayerState::Idle);
            }
        } else {
            if animation.animation_id != land {
                commands.entity(entity).insert(PlayerState::Run);
            }
        }
    }

    let air_friction = 1.0;
    // let air_friction = if output.grounded { 1.0 } else { 3.0 };

    match controller.translation {
        Some(vec) => {
            controller.translation = Some(Vec2::new(movement.x / air_friction, vec.y));
            // update if it already exists
        }
        None => {
            controller.translation = Some(Vec2::new(movement.x / air_friction, 0.0));
        }
    }
}

pub fn jump(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) {
    if query.is_empty() {
        return;
    }

    let (entity, output) = query.single();

    if input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp, KeyCode::Space]) && output.grounded {
        commands
            .entity(entity)
            .insert(Jump(0.0))
            .insert(PlayerState::Jump);
    } else {
        for gamepad in gamepads.iter() {
            if (button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)))
                && output.grounded
            {
                commands
                    .entity(entity)
                    .insert(Jump(0.0))
                    .insert(PlayerState::Jump);
            };
        }
    }
}

pub fn jump_release(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    query: Query<(Entity, &KinematicCharacterControllerOutput), With<KinematicCharacterController>>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, output) = query.single();

    // TODO: add buffer, maybe using a different Jump component ?
    if input.any_just_released([KeyCode::KeyW, KeyCode::ArrowUp, KeyCode::Space])
        && !output.grounded
    {
        commands.entity(entity).remove::<Jump>();
    } else {
        for gamepad in gamepads.iter() {
            if (button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::South)))
                && !output.grounded
            {
                commands.entity(entity).remove::<Jump>();
            };
        }
    }
}

pub fn rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut KinematicCharacterController,
        &mut Jump,
        &Player,
    )>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut controller, mut jump, player) = query.single_mut();

    let mut movement = time.delta().as_secs_f32() * player.jump_force;

    if movement + jump.0 >= player.max_jump_height {
        movement = player.max_jump_height - jump.0;
        commands.entity(entity).remove::<Jump>();
    }

    jump.0 += movement;

    match controller.translation {
        Some(vec) => controller.translation = Some(Vec2::new(vec.x, movement)),
        None => controller.translation = Some(Vec2::new(0.0, movement)),
    }
}

pub fn fall(
    time: Res<Time>,
    mut query: Query<(&mut KinematicCharacterController, &Player), (Without<Jump>, Without<Dash>)>,
) {
    if query.is_empty() {
        return;
    }

    let (mut controller, player) = query.single_mut();

    // I am using two-thirds of the Y-velocity since I want the character to fall slower than it rises
    let movement = time.delta().as_secs_f32() * (player.jump_force / 1.5) * -1.0;

    match controller.translation {
        Some(vec) => controller.translation = Some(Vec2::new(vec.x, movement)),
        None => controller.translation = Some(Vec2::new(0.0, movement)),
    }
}

pub fn update_direction(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), Without<Dash>>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();

    if output.desired_translation.x > 0.0 {
        commands.entity(player).insert(Direction::Right);
    } else if output.desired_translation.x < 0.0 {
        commands.entity(player).insert(Direction::Left);
    }
}

pub fn dash(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    // mouse: Res<ButtonInput<MouseButton>>,
    query: Query<(Entity, &KinematicCharacterController), Without<Dash>>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, controller) = query.single();

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
            commands
                .entity(entity)
                .remove::<Jump>()
                .insert(PlayerState::Dash)
                .insert(Dash {
                    initial_position: controller.translation.unwrap(),
                    value: 0.0,
                    direction
                });
        }
    }
}

pub fn dashing(
    mut query: Query<(
        Entity,
        &mut KinematicCharacterController,
        &Player,
        &mut Dash,
    )>,
    mut commands: Commands,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut controller, player, mut dash) = query.single_mut();

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

    if dash
        .initial_position
        .distance(dash.initial_position * movement * player.max_dash_length)
        >= player.max_dash_length / 100.0
    {
        // movement = player.max_dash_length - dash.value;
        commands
            .entity(entity)
            .remove::<Dash>();
    }

    dash.value += player.dash_speed;

    controller.translation = Some(movement * player.dash_speed)
}
