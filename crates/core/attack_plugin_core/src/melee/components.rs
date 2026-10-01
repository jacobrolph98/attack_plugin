use bevy::prelude::*;

#[derive(Component)]
pub struct Meleer {
    pub melee_type: Melee,
    pub range: f32,
    pub angle_radians: f32
}

#[derive(Component)]
pub struct MeleeStage{
    pub stage: u32,
    pub time_next_ray: f32
}

pub enum Melee {
    RaySweep {
        count: u32,
        interval: f32
    },
    Future
}