use bevy::prelude::*;

use crate::sweep::types::*;

#[derive(Component)]
pub struct Sweeper(pub Sweep);

#[derive(Component)]
pub struct SweepStage{
    pub stage: usize,
    pub time_next_ray: f32
}

#[derive(Component)]
pub struct DestroyOnSwept;