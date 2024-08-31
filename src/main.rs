mod aabb;
mod bvh;
mod forces;

use crate::forces::{AddForcesEachFrame, UpdateForce};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::{
    dynamics::{ExternalForce, ExternalImpulse, ReadMassProperties, RigidBody},
    geometry::{Collider, ColliderMassProperties},
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bvh::{LargeMass, SimulateGravity};

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0),
            RapierDebugRenderPlugin::default(),
            AddForcesEachFrame,
            SimulateGravity,
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, player_force1.in_set(UpdateForce))
        .add_systems(Update, player_force2.in_set(UpdateForce))
        .run();
}

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // cmd.spawn((Camera2dBundle::default(), PlayerCamera));

    let rad = 18.;

    // cmd.spawn((MaterialMesh2dBundle {
    //     mesh: meshes
    //         .add(shape::Quad::new(Vec2::splat(10.0)).into())
    //         .into(),
    //     material: materials.add(ColorMaterial::from(Color::WHITE)),
    //     transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
    //     ..default()
    // },));

    cmd.spawn((
        Player,
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(rad).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(rad),
        ColliderMassProperties::Density(1.0),
        ExternalForce::default(),
        ExternalImpulse::default(),
        ReadMassProperties::default(),
        LargeMass,
    ))
    .with_children(|parent| {
        parent.spawn(Camera2dBundle::default());
    });

    cmd.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(rad).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(rad),
        ColliderMassProperties::Density(1.0),
        ExternalForce::default(),
        ExternalImpulse::default(),
        ReadMassProperties::default(),
        LargeMass,
    ));
}

fn player_force1(mut bodies: Query<(&Transform, &mut ExternalForce), With<Player>>) {
    for (transform, mut ext_force) in bodies.iter_mut() {
        let force = Vec2::new(100.0, 0.0);
        ext_force.force += force;
    }
}

fn player_force2(mut bodies: Query<(&Transform, &mut ExternalForce), With<Player>>) {
    for (transform, mut ext_force) in bodies.iter_mut() {
        let force = Vec2::new(-200.0, 0.0);
        ext_force.force += force;
        ext_force.torque = 0.05;
    }
}
