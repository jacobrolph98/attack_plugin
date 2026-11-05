use bevy::prelude::*;

#[derive(Message)]
pub struct HitMessage {
    pub target: Entity,
    pub source: Option<Entity>,
    pub distance: f32,
    // TODO normal
}