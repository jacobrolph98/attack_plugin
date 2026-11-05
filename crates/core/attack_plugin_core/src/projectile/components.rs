use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone)]
#[require(ProjectileSpeed)]
pub struct Projectile {
    pub source: Option<Entity>,
}

#[derive(Component)]
pub struct ProjectileSpeed(pub f32);


impl Default for ProjectileSpeed {
    fn default() -> Self {
        ProjectileSpeed(1.)
    }
}

#[derive(Component)]
pub struct ProjectileDistance(pub f32);