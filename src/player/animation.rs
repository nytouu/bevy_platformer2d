use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use super::*;

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

pub fn land(
    mut commands: Commands,
    query: Query<(
        Entity,
        &SpritesheetAnimation,
        &KinematicCharacterControllerOutput,
    )>,
    library: Res<SpritesheetLibrary>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, animation, output) = query.single();

    let jump = library.animation_with_name("player_jump").unwrap();
    let air = library.animation_with_name("player_air").unwrap();

    if (animation.animation_id == jump || animation.animation_id == air) && output.grounded {
        commands.entity(entity).insert(PlayerState::Land);
    }
}
