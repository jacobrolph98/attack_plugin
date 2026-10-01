use bevy::prelude::*;

#[derive(Message)]
pub struct AttackMessage {
    pub source: Entity
}

#[derive(Message)]
pub struct HitMessage {
    pub target: Entity,
    pub source: Entity,
    // TODO normal
}