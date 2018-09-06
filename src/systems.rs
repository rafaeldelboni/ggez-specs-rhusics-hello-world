use ggez::event;
use ggez::graphics;
use ggez::{Context};

use rhusics_core::{Pose};
use rhusics_ecs::physics2d::{BodyPose2, RigidBodyParts2};
use cgmath::{Vector2};

use specs::{Join, ReadStorage, System, WriteStorage};

use components::{Square, Velocity, Controlable};

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        RigidBodyParts2<'a, f32, BodyPose2<f32>, ()>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Square>
    );

    fn run(&mut self, (mut rigid_body_parts, vel, mut square): Self::SystemData) {
        for (body, mut forces, vel, square) in (
            &rigid_body_parts.poses,
            &mut rigid_body_parts.forces,
            &vel,
            &mut square
        ).join(){
            let pos = body.position();

            square.position.x = pos.x;
            square.position.y = pos.y;

            forces.add_force(
                Vector2::new(vel.x * 10., vel.y * 10.)
            );

            println!("{:?} {:?} {:?} {:?}", square.position, pos, forces, vel);
        };
    }
}

pub struct RenderingSystem<'c> {
    ctx: &'c mut Context,
}

impl<'c> RenderingSystem<'c> {
    pub fn new(ctx: &'c mut Context) -> RenderingSystem<'c> {
        RenderingSystem { ctx }
    }
}

impl<'a, 'c> System<'a> for RenderingSystem<'c> {
    type SystemData = ReadStorage<'a, Square>;

    fn run(&mut self, texts: Self::SystemData) {
        &texts.join().for_each(|square| {
            graphics::rectangle(
                self.ctx,
                graphics::DrawMode::Line(1.0),
                graphics::Rect::new(
                    square.position.x,
                    square.position.y,
                    square.body_shape.x,
                    square.body_shape.y
                )
            ).unwrap();
        });
    }
}

pub struct ControlSystem {
    keycode: event::Keycode,
    down_event: bool,
}

impl ControlSystem {
    pub fn new(keycode: event::Keycode, down_event: bool) -> ControlSystem {
        ControlSystem { keycode, down_event }
    }
}

impl<'a> System<'a> for ControlSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Controlable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut velocities, controlables) = data;
        for (vel, _control) in (&mut velocities, &controlables).join() {
            match self.down_event {
                true =>
                    match self.keycode {
                        event::Keycode::Up => vel.y = -20.0,
                        event::Keycode::Down => vel.y = 20.0,
                        event::Keycode::Left => vel.x = -20.0,
                        event::Keycode::Right => vel.x = 20.0,
                        _ => {}
                    }
                false =>
                    match self.keycode {
                        event::Keycode::Up => vel.y = 0.0,
                        event::Keycode::Down => vel.y = 0.0,
                        event::Keycode::Left => vel.x = 0.0,
                        event::Keycode::Right => vel.x = 0.0,
                        _ => {}
                    }
            }
        }
    }
}

