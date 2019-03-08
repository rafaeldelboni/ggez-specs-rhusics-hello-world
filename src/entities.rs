use specs::{Builder, World};
use rhusics_core::{RigidBody, Pose};
use rhusics_ecs::{WithRigidBody};
use rhusics_ecs::collide2d::{
    BodyPose2,
    CollisionStrategy,
    CollisionShape2,
    CollisionMode};
use rhusics_ecs::physics2d::{
    Rectangle, Mass2, Velocity2};
use cgmath::{Basis2, One, Point2, Vector2};

use components::{Controllable};

pub fn create_static(world: &mut World) {
    world
        .create_entity()
        .with_static_rigid_body(
            CollisionShape2::<f32, BodyPose2<f32>, ()>::new_simple(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(100., 100.).into(),
            ),
            BodyPose2::new(Point2::new(70., 70.), Basis2::one()),
            RigidBody::default(),
            Mass2::new(1.),
        )
        .build();
}

pub fn create_moving(world: &mut World) {
    world
        .create_entity()
        .with_dynamic_rigid_body(
            CollisionShape2::<f32, BodyPose2<f32>, ()>::new_simple(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(400., 100.).into(),
            ),
            BodyPose2::new(Point2::new(200., 200.), Basis2::one()),
            Velocity2::new(Vector2::new(50.0, 50.0), 0.0),
            RigidBody::default(),
            Mass2::new(1.),
        )
        .build();
}

pub fn create_player(world: &mut World) {
    world
        .create_entity()
        .with_dynamic_rigid_body(
            CollisionShape2::<f32, BodyPose2<f32>, ()>::new_simple(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(200., 100.).into(),
            ),
            BodyPose2::new(Point2::new(100., 400.), Basis2::one()),
            Velocity2::new(Vector2::new(0.0, 0.0), 0.0),
            RigidBody::default(),
            Mass2::new(1.),
        )
        .with(Controllable { x: 0., y: 0. })
        .build();
}
