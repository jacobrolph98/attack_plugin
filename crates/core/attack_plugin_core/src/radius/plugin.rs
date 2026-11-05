use bevy::prelude::*;

use crate::{radius::{messages::*, systems::*}, sets::*};

pub struct RadiusPlugin;

impl Plugin for RadiusPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RadiusCheckMessage>();
        #[cfg(feature = "2d")]
        app.add_systems(FixedUpdate, radius_check_2d.in_set(AttackSet::Process));
        #[cfg(feature = "3d")]
        app.add_systems(FixedUpdate, radius_check_3d.in_set(AttackSet::Process));
    }
}