use bevy::prelude::*;

/// Check targets within a range of a given radius of a position, triggering a Hit Message
#[derive(Message)]
pub struct RadiusCheckMessage {
    pub pos: Vec3,
    pub radius: f32,
    pub line_of_sight: bool,
    pub source: Option<Entity>
}