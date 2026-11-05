use bevy::prelude::*;
use crate::{sets::AttackSet, sweep::{messages::*, systems::*}};

pub struct SweepPlugin;

impl Plugin for SweepPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<StartSweep>()
            .add_systems(FixedUpdate, (
                start_sweep.in_set(AttackSet::Intent),
                #[cfg(feature = "2d")]
                ray_sweep_2d.in_set(AttackSet::Process),
                #[cfg(feature = "3d")]
                ray_sweep_3d.in_set(AttackSet::Process),
            ))
            .add_observer(end_sweep)
            ;
    }
}