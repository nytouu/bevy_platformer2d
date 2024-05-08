use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
    mut query: Query<(
        Entity,
        &mut KinematicCharacterController,
        &KinematicCharacterControllerOutput,
        &Player,
        &PlayerState,
    )>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut controller, output, player, state) = query.single_mut();

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
        if movement == Vec2::ZERO {
            commands.entity(entity).insert(PlayerState::Idle);
        } else {
            commands.entity(entity).insert(PlayerState::Run);
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

    let (player, output) = query.single();

    if input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp, KeyCode::Space]) && output.grounded {
        commands
            .entity(player)
            .insert(Jump(0.0))
            .insert(PlayerState::Jump);
    } else {
        for gamepad in gamepads.iter() {
            if (button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)))
                && output.grounded
            {
                commands
                    .entity(player)
                    .insert(Jump(0.0))
                    .insert(PlayerState::Jump);
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
    mut query: Query<(&mut KinematicCharacterController, &Player), Without<Jump>>,
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
    query: Query<(Entity, &KinematicCharacterControllerOutput)>,
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

pub fn update_sprite_direction(mut query: Query<(&mut Sprite, &Direction)>) {
    if query.is_empty() {
        return;
    }

    let (mut sprite, direction) = query.single_mut();

    match direction {
        Direction::Right => sprite.flip_x = true,
        Direction::Left => sprite.flip_x = false,
    }
}
