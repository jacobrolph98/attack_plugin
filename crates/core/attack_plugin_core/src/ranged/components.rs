use bevy::prelude::*;

#[derive(Component)]
#[require(Transform)]
pub struct Shooter {
    pub projectile_speed: f32
}

#[derive(Component)]
#[require(ProjectileSpeed)]
pub struct Projectile {
    pub source: Entity,
}

#[derive(Component)]
pub struct ProjectileSpeed(pub f32);

impl Default for ProjectileSpeed {
    fn default() -> Self {
        ProjectileSpeed(1.)
    }
}