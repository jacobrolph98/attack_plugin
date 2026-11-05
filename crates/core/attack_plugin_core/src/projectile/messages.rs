use bevy::prelude::*;

#[derive(Message)]
pub struct SpawnProjectile {
    pub pos: Vec3,
    pub direction: Quat,
    pub speed: f32,
    pub source: Option<Entity>
}