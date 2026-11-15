use bevy::prelude::*;

#[derive(Message)]
pub struct SpawnProjectile {
    pub pos: Vec3,
    pub direction: Quat,
    pub speed: f32,
    pub track: Option<(Vec3, f32)>, // Target pos, degrees per sec turn radius
    pub source: Option<Entity>
}