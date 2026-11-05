use bevy::prelude::*;

#[derive(Message)]
pub struct RadiusCheckMessage {
    pub pos: Vec3,
    pub radius: f32,
    pub line_of_sight: bool,
    pub source: Option<Entity>
}