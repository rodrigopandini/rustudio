use bevy::prelude::*;
use bevy_rapier3d::{
    dynamics::RigidBody,
    geometry::{Collider, Restitution},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
    }
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // floor
    let floor = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(20.0))),
            material: materials.add(Color::rgb(0.88, 0.84, 0.72).into()),
            ..default()
        },
        Name::new("Floor"),
    );

    commands.spawn(floor).insert((
        RigidBody::Fixed,
        Collider::cuboid(10.0, 0.01, 10.0),
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
    ));

    // cube01
    let cube01 = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::rgb(0.57, 0.47, 0.34).into()),
            ..default()
        },
        Name::new("Cube01"),
    );

    commands.spawn(cube01).insert((
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        Restitution::coefficient(0.7),
        TransformBundle::from(Transform::from_xyz(0.0, 5.0, 6.5)),
    ));

    // cube02
    let cube02 = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(0.5))),
            material: materials.add(Color::rgb(0.57, 0.47, 0.34).into()),
            ..default()
        },
        Name::new("Cube02"),
    );

    commands.spawn(cube02).insert((
        RigidBody::Dynamic,
        Collider::cuboid(0.25, 0.25, 0.25),
        Restitution::coefficient(0.7),
        TransformBundle::from(Transform::from_xyz(2.0, 5.0, 7.5)),
    ));
}
