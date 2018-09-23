use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Controlable {
    pub x: f32,
    pub y: f32,
}

impl Component for Controlable {
    type Storage = VecStorage<Self>;
}
