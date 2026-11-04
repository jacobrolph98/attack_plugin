use bevy::prelude::*;

use crate::{melee::plugin::MeleePlugin, messages::*, ranged::plugin::RangedPlugin, sets::AttackSet};

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        //#[cfg(feature = "ranged")]
        app.add_plugins(RangedPlugin);
        
        //#[cfg(feature = "melee")]
        app.add_plugins(MeleePlugin);
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
        
    }
}