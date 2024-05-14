use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;

pub mod camera;
pub mod player;
pub mod world;
pub mod egui;

use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;
use egui::EguiDockPlugin;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Celeste-like game thing".to_string(),
                        resolution: WindowResolution::new(1280.0, 720.0),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            // RapierDebugRenderPlugin::default(),
            CameraPlugin,
            WorldPlugin,
            PlayerPlugin,
            SpritesheetAnimationPlugin,
            EguiDockPlugin,
        ))
        .run();
}
