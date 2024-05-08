use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_rapier2d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size = Vec2::new(1000.0, 5.0);
    commands.spawn((
        ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(size.x, size.y))),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            transform: Transform::from_xyz(0.0, -20.0, 0.0),
            ..Default::default()
        },
        Collider::cuboid(size.x / 2.0, size.y / 2.0),
    ));
}
