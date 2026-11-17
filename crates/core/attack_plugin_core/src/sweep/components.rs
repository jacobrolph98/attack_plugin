use bevy::prelude::*;

use crate::sweep::types::*;

/// Component which holds data for sweeping while one is actively happening
#[derive(Component)]
pub struct Sweeper(pub Sweep);

/// Tracks which stage a sweep is in, and the time the next stage should trigger
#[derive(Component)]
pub struct SweepStage{
    pub stage: usize,
    pub time_next_ray: f32
}

/// Whether the sweeper should erase the entity it is attached to when done sweeping
#[derive(Component)]
pub struct DestroyOnSwept;