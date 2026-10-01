use bevy::prelude::*;

use crate::{melee::systems::*, messages::*, ranged::systems::*, sets::AttackSet};

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_plugins(ProtocolPlugin)
            .add_message::<AttackMessage>()
            .add_message::<HitMessage>()
            .configure_sets(FixedUpdate, (
                AttackSet::Intent, 
                AttackSet::Process, 
                AttackSet::Result
            ).chain())
            ;
        //#[cfg(feature = "ranged")]
        app.add_systems(FixedUpdate, (
            spawn_projectile.in_set(AttackSet::Intent),
            #[cfg(feature = "2d")]
            step_projectile_2d.in_set(AttackSet::Process),
            #[cfg(feature = "3d")]
            step_projectile_3d.in_set(AttackSet::Process)
        ));

        //#[cfg(feature = "melee")]
        app.add_systems(FixedUpdate, (
            start_melee.in_set(AttackSet::Intent),
            #[cfg(feature = "2d")]
            ray_sweep_2d.in_set(AttackSet::Process),
            #[cfg(feature = "3d")]
            ray_sweep_3d.in_set(AttackSet::Process)
        ));
    }
}