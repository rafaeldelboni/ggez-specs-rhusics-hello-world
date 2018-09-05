use ggez::graphics;

use specs::{Component, NullStorage, VecStorage};

#[derive(Debug)]
pub struct Square {
    pub body_shape: graphics::Point2,
    pub position: graphics::Point2,
}

impl Component for Square {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Controlable;

impl Component for Controlable {
    type Storage = NullStorage<Self>;
}

