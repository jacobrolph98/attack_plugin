use bevy::prelude::*;

use crate::{projectile::{messages::SpawnProjectile, systems::*}, sets::AttackSet};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnProjectile>();
        app.add_systems(FixedUpdate, spawn_projectile.in_set(AttackSet::Intent));
        app.add_systems(FixedUpdate, (
            rotate_projectile.in_set(AttackSet::Process),
            #[cfg(feature = "2d")]
            step_projectile_2d.in_set(AttackSet::Process),
            #[cfg(feature = "3d")]
            step_projectile_3d.in_set(AttackSet::Process)
        ).chain());
    }
}