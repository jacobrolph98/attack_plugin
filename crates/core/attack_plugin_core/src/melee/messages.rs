use bevy::prelude::*;

#[derive(Message)]
pub struct MeleeRadiusMessage {
    pub entity: Entity,
    pub line_of_sight: bool
}