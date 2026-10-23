use attack_plugin_core::ranged::components::Projectile;
use bevy::prelude::*;
use lightyear::prelude::*;
pub struct ProtocolPlugin;

/*
TODO register for replication;
Projectiles
Hits
*/

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.component::<Projectile>().replicate();
    }
}