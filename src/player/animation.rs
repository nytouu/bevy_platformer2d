use bevy::prelude::*;
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

pub fn land(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SpritesheetAnimation, &PlayerState)>,
    library: Res<SpritesheetLibrary>,
    mut events: EventReader<AnimationEvent>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut animation, state) = query.single_mut();

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

pub fn air(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SpritesheetAnimation, &PlayerState)>,
    library: Res<SpritesheetLibrary>,
    mut events: EventReader<AnimationEvent>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut animation, state) = query.single_mut();

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
