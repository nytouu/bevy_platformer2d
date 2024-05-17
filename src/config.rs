use bevy::prelude::*;
use bevy::window::{WindowPlugin, WindowResolution};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_window_icon)
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Celeste-like game thing".to_string(),
                            resolution: WindowResolution::new(1280.0, 720.0),
                            resizable: true,
                            ..default()
                        }),
                        ..Default::default()
                    })
                    .set(ImagePlugin::default_nearest())
                    .set(AssetPlugin {
                        watch_for_changes_override: Some(true),
                        ..Default::default()
                    }),
            )
            .insert_resource(Msaa::Off)
            .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.3)));
    }
}
fn set_window_icon() {}
