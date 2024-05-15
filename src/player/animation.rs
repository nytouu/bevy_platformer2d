use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use super::Direction;
use super::*;

use super::dash::DashCooldown;
use super::dash::DashTrail;

pub fn update_animation(
    mut query: Query<(&mut SpritesheetAnimation, &PlayerState)>,
    library: Res<SpritesheetLibrary>,
) {
    if query.is_empty() {
        return;
    }

    let (mut animation, state) = query.single_mut();

    match state {
        PlayerState::Idle => {
            if let Some(id) = library.animation_with_name("player_idle") {
                animation.animation_id = id;
            }
        }
        PlayerState::Run => {
            if let Some(id) = library.animation_with_name("player_run") {
                animation.animation_id = id;
            }
        }
        PlayerState::Dash => {
            if let Some(id) = library.animation_with_name("player_dash") {
                animation.animation_id = id;
            }
        }
        PlayerState::Climb => {
            if let Some(id) = library.animation_with_name("player_climb") {
                animation.animation_id = id;
            }
        }
        PlayerState::Wall => {
            if let Some(id) = library.animation_with_name("player_wall") {
                animation.animation_id = id;
            }
        }
        PlayerState::Jump => {
            if let Some(id) = library.animation_with_name("player_jump") {
                animation.animation_id = id;
            }
        }
        PlayerState::Air => {
            if let Some(id) = library.animation_with_name("player_air") {
                animation.animation_id = id;
            }
        }
        PlayerState::Land => {
            if let Some(id) = library.animation_with_name("player_land") {
                animation.animation_id = id;
            }
        }
    }
}

/// Transitionne depuis l'animation de land vers l'idle
pub fn land_to_idle(
    mut commands: Commands,
    query: Query<(Entity, &mut SpritesheetAnimation)>,
    library: Res<SpritesheetLibrary>,
    mut events: EventReader<AnimationEvent>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, animation) = query.single();

    let land = library.animation_with_name("player_land").unwrap();
    for event in events.read() {
        match event {
            AnimationEvent::AnimationCycleEnd { .. } => {
                if animation.animation_id == land {
                    commands.entity(entity).insert(PlayerState::Idle);
                }
            }
            _ => {}
        }
    }
}

/// Transitionne depuis l'animation de jump vers air
pub fn jump_to_air(
    mut commands: Commands,
    query: Query<(Entity, &SpritesheetAnimation)>,
    library: Res<SpritesheetLibrary>,
    mut events: EventReader<AnimationEvent>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, animation) = query.single();

    let jump = library.animation_with_name("player_jump").unwrap();
    for event in events.read() {
        match event {
            AnimationEvent::AnimationCycleEnd { .. } => {
                if animation.animation_id == jump {
                    commands.entity(entity).insert(PlayerState::Air);
                }
            }
            _ => {}
        }
    }
}

/// Lance l'animation de land
pub fn land(
    mut commands: Commands,
    query: Query<(Entity, &SpritesheetAnimation, &Player)>,
    library: Res<SpritesheetLibrary>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, animation, player) = query.single();

    let jump = library.animation_with_name("player_jump").unwrap();
    let air = library.animation_with_name("player_air").unwrap();

    if (animation.animation_id == jump || animation.animation_id == air) && player.grounded {
        commands.entity(entity).insert(PlayerState::Land);
    }
}

/// Met à jour la direction du sprite, ici je ne filtre pas les éléments avec le component Player
/// car ce système pourrait s'executer sur des ennemis par exemple
pub fn update_sprite_direction(mut query: Query<(&mut Sprite, &Direction)>) {
    if query.is_empty() {
        return;
    }

    for (mut sprite, direction) in &mut query {
        match direction {
            Direction::Right => sprite.flip_x = true,
            Direction::Left => sprite.flip_x = false,
        }
    }
}

/// Met à jour la couleur du sprite pendant le dash
pub fn update_dash_color(mut query: Query<&mut Sprite, With<Dash>>) {
    if query.is_empty() {
        return;
    }

    let mut sprite = query.single_mut();

    if sprite.color != Color::BLUE {
        sprite.color = Color::BLUE;
    }
}

/// Reset la couleur du dash après celui ci
pub fn reset_dash_color(
    mut query: Query<&mut Sprite, (Without<Dash>, Without<DashCooldown>, Without<DashTrail>)>,
) {
    if query.is_empty() {
        return;
    }

    let mut sprite = query.single_mut();

    if sprite.color != Color::WHITE {
        sprite.color = Color::WHITE;
    }
}

/// Met a jour l'état du joueur après le dash
pub fn post_dash(
    mut commands: Commands,
    // on peut query un Option, la query aura un None ou Some(&Dash)
    query: Query<(Entity, &SpritesheetAnimation, Option<&Dash>)>,
    library: Res<SpritesheetLibrary>,
) {
    // ici ci une entité a Entity et SpritesheetAnimation mais pas de Dash alors la query aura
    // quand même un élément puisque le dash sera None
    if query.is_empty() {
        return;
    }

    let (entity, animation, dashing) = query.single();

    let dash = library.animation_with_name("player_dash").unwrap();

    if (animation.animation_id == dash) && dashing.is_none() {
        commands.entity(entity).insert(PlayerState::Air);
    }
}
