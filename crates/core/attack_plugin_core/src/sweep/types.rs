use bevy::prelude::*;

pub enum Attachment {
    Entity(Entity),
    Transform(Transform)
}

#[derive(Clone)]
pub struct Sweep {
    pub stages: usize,
    pub interval: f32,
    pub range: f32,
    pub formula: fn(usize) -> Quat, // Input stage -> output offset angle
}