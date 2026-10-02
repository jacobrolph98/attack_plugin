use bevy::prelude::*;

#[derive(Component)]
pub struct Meleer {
    pub melee_type: Melee,
    pub range: f32,
}

#[derive(Component)]
pub struct MeleeStage{
    pub stage: usize,
    pub time_next_ray: f32
}

pub enum Melee {
    RaySweep {
        count: usize,
        interval: f32,
        formula: fn(usize) -> Quat // Input stage, output offset angle
    },
    Radius {
        line_of_sight: bool
    }
}