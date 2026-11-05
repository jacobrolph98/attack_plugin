use bevy::prelude::*;

use crate::sweep::types::*;

#[derive(Message)]
pub struct StartSweep {
    pub attach: Attachment,
    pub stats: Sweep
}