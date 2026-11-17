use bevy::prelude::*;

/// Written when a target is hit by an entity
#[derive(Message)]
pub struct HitMessage {
    pub target: Entity,
    pub source: Option<Entity>,
    pub distance: f32,
    // TODO normal
}