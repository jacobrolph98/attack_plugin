use bevy::prelude::*;

use crate::{
    messages::*, 
    sets::AttackSet, 
    projectile::plugin::ProjectilePlugin, 
    radius::plugin::RadiusPlugin, 
    sweep::plugin::SweepPlugin
};

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<HitMessage>()
            .configure_sets(FixedUpdate, (
                AttackSet::Intent, 
                AttackSet::Process, 
                AttackSet::Result
            ).chain())
            ;
        app.add_plugins((ProjectilePlugin, RadiusPlugin, SweepPlugin));
    }
}