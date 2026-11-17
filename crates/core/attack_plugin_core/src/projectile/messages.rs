use bevy::prelude::*;

/// Spawn a projectile which will step a direction until hitting something, triggering a Hit Message
#[derive(Message)]
pub struct SpawnProjectile {
    pub pos: Vec3,
    pub direction: Quat,
    pub speed: f32,
    pub track: Option<(Vec3, f32)>, // Target pos, degrees per sec turn radius
    pub source: Option<Entity>
}