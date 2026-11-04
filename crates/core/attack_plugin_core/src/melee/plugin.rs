use bevy::prelude::*;

use crate::{melee::{messages::MeleeRadiusMessage, systems::*}, sets::AttackSet};

pub struct MeleePlugin;

impl Plugin for MeleePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            start_melee.in_set(AttackSet::Intent),

            #[cfg(feature = "2d")]
            ray_sweep_2d.in_set(AttackSet::Process),

            #[cfg(feature = "3d")]
            ray_sweep_3d.in_set(AttackSet::Process),

            #[cfg(feature = "2d")]
            radius_check_2d.in_set(AttackSet::Process),
            
            #[cfg(feature = "3d")]
            radius_check_3d.in_set(AttackSet::Process)
        ));
        app.add_message::<MeleeRadiusMessage>();
    }
}