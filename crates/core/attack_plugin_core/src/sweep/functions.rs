use bevy::prelude::*;
use crate::sweep::{components::*, types::*};

pub fn sweep_bundle(sweep: Sweep, time: f32) -> impl Bundle {
    (
        Sweeper(sweep),
        SweepStage {
            stage: 0,
            time_next_ray: time 
        }
    )
}