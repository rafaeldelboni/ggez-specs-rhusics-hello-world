use ggez::event;
use ggez::graphics;
use ggez::{Context};

use rhusics_ecs::physics2d::{BodyPose2, RigidBodyParts2};
use cgmath::{Vector2};

use specs::{Join, ReadStorage, System, WriteStorage};

use components::{Controllable};

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        RigidBodyParts2<'a, f32, BodyPose2<f32>, ()>,
        ReadStorage<'a, Controllable>
    );

    fn run(&mut self, (mut rigid_body_parts, control): Self::SystemData) {
        for (mut forces, control) in (
            &mut rigid_body_parts.forces,
            &control
        ).join(){
            forces.add_force(Vector2::new(control.x, control.y));
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
    type SystemData = RigidBodyParts2<'a, f32, BodyPose2<f32>, ()>;

    fn run(&mut self, bodies: Self::SystemData) {
        for shape in (&bodies.shapes).join() {
            let shape = shape.bound();
            graphics::rectangle(
                self.ctx,
                graphics::DrawMode::Line(1.0),
                graphics::Rect::new(
                    shape.min.x,
                    shape.min.y,
                    shape.max.x - shape.min.x,
                    shape.max.y - shape.min.y
                )
            ).expect("Error drawing entity bounds!");
        };
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
        WriteStorage<'a, Controllable>
    );

    fn run(&mut self, data: Self::SystemData) {
        let mut controllables = data;
        for control in (&mut controllables).join() {
            match self.down_event {
                true =>
                    match self.keycode {
                        event::Keycode::Up => control.y = -200.0,
                        event::Keycode::Down => control.y = 200.0,
                        event::Keycode::Left => control.x = -200.0,
                        event::Keycode::Right => control.x = 200.0,
                        _ => {}
                    }
                false =>
                    match self.keycode {
                        event::Keycode::Up => control.y = 0.0,
                        event::Keycode::Down => control.y = 0.0,
                        event::Keycode::Left => control.x = 0.0,
                        event::Keycode::Right => control.x = 0.0,
                        _ => {}
                    }
            }
        }
    }
}

