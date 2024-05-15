use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_rapier2d::prelude::*;

/// Met en place des éléments statiques et fixes dans le monde
pub struct WorldPlugin;

// le keyword impl spécifie que T implémente des méthodes spécifiées dans le bloc de code
// on peut aussi préciser un implémentation pour un "trait" (c'est une interface c# en gros)
// le trait Plugin veut qu'on implémente la méthode "build", bevy souhaite qu'on rajoute nos
// systèmes a l'application donc on rajoute ici le setup du monde au "Startup"
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    // les Commands permenttent d'intéragir avec le monde bevy, notamment pour faire spawn des
    // entités ou rajouter des components a celles-ci
    mut commands: Commands,
    // ici on récupère une "ressource" mutable, on va rajouter des choses donc on spécifie qu'elle
    // est mutable
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size = Vec2::new(300.0, 5.0);
    // on peut faire spawn une entités avec la méthode spawn
    // elle prend un seul argument, un "Bundle" qui peut être soit un component seul, ou une
    // collection de components
    commands.spawn((
        RigidBody::Fixed,
        ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(size.x, size.y))),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            transform: Transform::from_xyz(0.0, -20.0, 0.0),
            ..Default::default()
        },
        Collider::cuboid(size.x / 2.0, size.y / 2.0),
    ));
}
