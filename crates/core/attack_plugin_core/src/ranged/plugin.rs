use bevy::prelude::*;

use crate::{ranged::systems::*, sets::AttackSet};

pub struct RangedPlugin;

impl Plugin for RangedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            spawn_projectile.in_set(AttackSet::Intent),
            #[cfg(feature = "2d")]
            step_projectile_2d.in_set(AttackSet::Process),
            #[cfg(feature = "3d")]
            step_projectile_3d.in_set(AttackSet::Process)
        ));
    }
}