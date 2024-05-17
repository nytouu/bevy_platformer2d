use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::player::{*, Direction};

const ANIMATION_DURATION: u32 = 60;

pub fn setup_player(
    mut commands: Commands,
    // Ressource donnée par la lib de spritesheetanimation
    // permet de push en ressource
    mut library: ResMut<SpritesheetLibrary>,
    // permet de spécifier le layout de la spritesheet
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    // l'AssetServer fait le lien avec le dossier "assets" dans la racine
    // nécessite la feature "file_watcher" de bevy dans le Cargo.toml (je crois)
    assets: Res<AssetServer>,
) {
    // load asset for sprite
    let player: Handle<Image> = assets.load("player.png");

    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(32.0, 32.0),
        8,
        8,
        None,
        None,
    ));

    // importe tout les clip pour les animations
    // run
    let run_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(8, 8).row(0));
    });

    let run_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(run_clip_id.into())
            .set_duration(AnimationDuration::PerFrame(ANIMATION_DURATION))
            .set_repeat(AnimationRepeat::Loop);
    });

    library.name_animation(run_anim_id, "player_run").unwrap();

    // idle
    let idle_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(8, 8).row(1));
    });

    let idle_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(idle_clip_id.into())
            .set_duration(AnimationDuration::PerFrame(ANIMATION_DURATION))
            .set_repeat(AnimationRepeat::Loop);
    });

    library.name_animation(idle_anim_id, "player_idle").unwrap();

    // climb
    let climb_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(8, 8).row(2));
    });

    let climb_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(climb_clip_id.into())
            .set_duration(AnimationDuration::PerFrame(ANIMATION_DURATION))
            .set_repeat(AnimationRepeat::Loop);
    });

    library
        .name_animation(climb_anim_id, "player_climb")
        .unwrap();

    // air
    let air_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(8, 8).row(3));
    });

    let air_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(air_clip_id.into())
            .set_duration(AnimationDuration::PerFrame(ANIMATION_DURATION))
            .set_repeat(AnimationRepeat::Loop);
    });

    library.name_animation(air_anim_id, "player_air").unwrap();

    // jump
    let jump_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(8, 8).horizontal_strip(0, 4, 6));
    });

    let jump_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(jump_clip_id.into())
            .set_duration(AnimationDuration::PerFrame(ANIMATION_DURATION))
            .set_repeat(AnimationRepeat::Cycles(1));
    });

    library.name_animation(jump_anim_id, "player_jump").unwrap();

    // land
    let land_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(8, 8).horizontal_strip(0, 5, 6));
    });

    let land_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(land_clip_id.into())
            .set_duration(AnimationDuration::PerFrame(ANIMATION_DURATION))
            .set_repeat(AnimationRepeat::Cycles(1));
    });

    library.name_animation(land_anim_id, "player_land").unwrap();

    // wall
    let wall_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(8, 8).horizontal_strip(0, 6, 1));
    });

    let wall_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(wall_clip_id.into())
            .set_duration(AnimationDuration::PerFrame(ANIMATION_DURATION))
            .set_repeat(AnimationRepeat::Cycles(1));
    });

    library.name_animation(wall_anim_id, "player_wall").unwrap();

    // dash
    let dash_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(8, 8).horizontal_strip(0, 7, 6));
    });

    let dash_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(dash_clip_id.into())
            .set_duration(AnimationDuration::PerFrame(ANIMATION_DURATION))
            .set_repeat(AnimationRepeat::Cycles(1));
    });

    library.name_animation(dash_anim_id, "player_dash").unwrap();

    // spawn player
    commands.spawn((
        // Name permet de donner un nom a l'entitée, elle nommera l'entité dans l'inspecteur aussi
        Name::new("Player"),
        SpriteSheetBundle {
            texture: player,
            atlas: TextureAtlas {
                layout,
                ..Default::default()
            },
            ..Default::default()
        },
        // player data
        Player::default(),
        // player state
        PlayerState::Idle,
        // current animation
        SpritesheetAnimation::from_id(idle_anim_id),
        // player direction utilisée pour le flip du sprite
        Direction::Right,
        // physics
        // colliders : https://rapier.rs/docs/user_guides/bevy_plugin/colliders
        Collider::capsule_y(4.0, 4.0),
        Damping {
            linear_damping: 1.5,
            ..Default::default()
        },
        // rigidbodies : https://rapier.rs/docs/user_guides/bevy_plugin/rigid_bodies
        RigidBody::Dynamic,
        Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        },
        Ccd::enabled(),
        GravityScale(GRAVITY_SCALE),
        LockedAxes::ROTATION_LOCKED,
    ));
}
