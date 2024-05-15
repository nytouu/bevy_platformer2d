use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

// l'enum Direction existe dans ce namespace donc je le remplace par celui que j'ai défini dans
// player/mod.rs cet override doit être explicite, même quand on importe *
// super fait ici réference au module supérieur, ici à player/mod.rs
// dans la hiérarchie de rust, movement appartient à player
use super::Direction;
use super::*;

/// Gauche droite bouger movement
pub fn strafe(
    // les boutons sont des ressources fournies par bevy, il nous suffit de les lire
    input: Res<ButtonInput<KeyCode>>,
    // time est hyper pratique pour gérer, récup le temps dans le système
    time: Res<Time>,
    // les manettes sont des ressources données par bevy aussi
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Velocity, &Player, &PlayerState), Without<Dash>>,
    library: Res<SpritesheetLibrary>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut velocity, player, state) = query.single_mut();

    // Mouvement qui sera appliqué au player après avoir process les inputs et son state
    let mut movement: f32 = 0.0;

    // doc :
    // https://docs.rs/bevy/0.13.0/bevy/input/keyboard/enum.KeyCode.html
    // https://bevy-cheatbook.github.io/input/keyboard.html
    if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        movement += time.delta_seconds() * player.speed;
    }

    if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        movement += time.delta_seconds() * player.speed * -1.0;
    }

    // si pas de movement clavier, on check la manette
    // le clavier a la prio parce que why not
    if movement == 0.0 {
        // doc : https://bevy-cheatbook.github.io/input/gamepad.html
        for gamepad in gamepads.iter() {
            let left_stick_x = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .unwrap();
            if left_stick_x.abs() > JOYSTICK_THRESHOLD {
                movement += time.delta_seconds() * player.speed * left_stick_x;
            }
            // Dpad
            if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadRight)) {
                movement += time.delta_seconds() * player.speed;
            }
            if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft)) {
                movement += time.delta_seconds() * player.speed * -1.0;
            }
        }
    }

    if player.grounded {
        if movement == 0.0 {
            if *state != PlayerState::Land {
                // le fait de rajouter un component déjà présent dans l'entité fait qu'elle
                // remplace celle déjà existante
                commands.entity(entity).insert(PlayerState::Idle);
            }
        } else {
            if *state != PlayerState::Land {
                commands.entity(entity).insert(PlayerState::Run);
            }
        }

        velocity.linvel.x = movement;
    } else {
        velocity.linvel.x += movement / AIR_FRICTION;
        velocity.linvel.x = velocity
            .linvel
            .x
            .clamp(-player.speed / 150.0, player.speed / 100.0);
    }
}

pub fn jump(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    query: Query<(Entity, &Player), Without<Jump>>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, player) = query.single();

    if input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp, KeyCode::Space]) && player.grounded {
        commands
            .entity(entity)
            .insert(Jump(0.0))
            .insert(PlayerState::Jump);
    } else {
        for gamepad in gamepads.iter() {
            if (button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)))
                && player.grounded
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
    query: Query<(Entity, &Player)>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, player) = query.single();

    // TODO: add buffer, maybe using a different Jump component ?
    if input.any_just_released([KeyCode::KeyW, KeyCode::ArrowUp, KeyCode::Space])
        && !player.grounded
    {
        commands.entity(entity).remove::<Jump>();
    } else {
        for gamepad in gamepads.iter() {
            if (button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::South)))
                && !player.grounded
            {
                commands.entity(entity).remove::<Jump>();
            };
        }
    }
}

pub fn rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Velocity, &mut Jump, &Player)>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut velocity, mut jump, player) = query.single_mut();

    let mut movement = time.delta_seconds() * player.jump_force;

    if movement + jump.0 >= player.max_jump_height {
        movement = player.max_jump_height - jump.0;
        commands.entity(entity).remove::<Jump>();
    }

    // FIXME: i hate delta
    jump.0 += movement / 100.0;

    velocity.linvel.y = movement;
}

pub fn update_direction(mut commands: Commands, query: Query<(Entity, &Velocity), Without<Dash>>) {
    if query.is_empty() {
        return;
    }

    let (player, velocity) = query.single();

    if velocity.linvel.x > 0.0 {
        commands.entity(player).insert(Direction::Right);
    } else if velocity.linvel.x < 0.0 {
        commands.entity(player).insert(Direction::Left);
    }
}

pub fn check_for_ground(
    mut query: Query<(Entity, &mut Player, &Transform)>,
    rapier_context: Res<RapierContext>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut player, transform) = query.single_mut();

    let ray_pos = Vec2::new(transform.translation.x, transform.translation.y);
    let ray_dir = Vec2::new(0.0, -1.0);
    let max_toi = 8.0; // INFO: should be the height of the player (collider's halfsize * 2)
    let solid = true;
    let filter = QueryFilter::exclude_dynamic()
        .exclude_sensors()
        .exclude_rigid_body(entity);

    if rapier_context
        .cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
        .is_some()
    {
        player.grounded = true;
    } else {
        player.grounded = false;
    }
}
