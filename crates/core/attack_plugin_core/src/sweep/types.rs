use bevy::{ecs::query::QueryData, prelude::*};

use crate::sweep::components::*;

pub enum Attachment {
    Entity(Entity),
    Transform(Transform)
}

#[derive(Clone)]
pub struct Sweep {
    pub stages: usize,
    pub interval: f32,
    pub range: f32,
    pub formula: fn(usize) -> Quat, // Input stage -> output offset angle
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SweepQuery {
    pub entity: Entity,
    pub transform: &'static Transform,
    pub sweeper: &'static Sweeper,
    pub stage: &'static mut SweepStage,
    pub source: Option<&'static SweepSource>,
}