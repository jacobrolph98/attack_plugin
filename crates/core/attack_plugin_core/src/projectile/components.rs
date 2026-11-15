use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marker component that contains reference to source entity, if any
#[derive(Component, Serialize, Deserialize, Clone)]
#[require(ProjectileSpeed)]
pub struct Projectile {
    pub source: Option<Entity>,
}

/// Units per second for projectile to travel
#[derive(Component)]
pub struct ProjectileSpeed(pub f32);

impl Default for ProjectileSpeed {
    fn default() -> Self {
        ProjectileSpeed(1.)
    }
}

/// Distance projectile has travelled, incremented per step
#[derive(Component)]
pub struct ProjectileDistance(pub f32);

/// Max degrees turnable per second
#[derive(Component)]
pub struct ProjectileTurnRate(pub f32);

impl Default for ProjectileTurnRate {
    fn default() -> Self {
        ProjectileTurnRate(90.)
    }
}

/// Target position for projectile to turn toward
#[derive(Component)]
#[require(ProjectileTurnRate)]
pub struct ProjectileTrackPosition(pub Vec3);