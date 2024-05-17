use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_rapier2d::prelude::*;

pub struct TileWorldPlugin;

mod helpers;

impl Plugin for TileWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(helpers::TiledMapPlugin)
            .add_systems(Startup, (setup_tiled_map, setup_colliders).chain());
    }
}

fn setup_tiled_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<helpers::TiledMap> = asset_server.load("map.tmx");

    commands.spawn(helpers::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}

fn setup_colliders(mut commands: Commands, mut query: Query<(Entity, &TilePos)>) {
    if query.is_empty() {
        return;
    }

    for (entity, mut tile) in &mut query {
        commands.entity(entity).insert(Collider::cuboid(0.5, 0.5));
    }
}
