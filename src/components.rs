use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Controllable {
    pub x: f32,
    pub y: f32,
}

impl Component for Controllable {
    type Storage = VecStorage<Self>;
}
