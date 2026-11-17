use bevy::prelude::*;
use crate::messages::*;
use crate::sweep::{components::*, functions::*, messages::*, types::*};

pub fn start_sweep(
    mut sweep_r: MessageReader<StartSweep>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for sweep in sweep_r.read() {
        match sweep.attach {
            Attachment::Entity(entity) => {
                commands.entity(entity).insert(
                    sweep_bundle(sweep.stats.clone(), time.elapsed_secs())
                );
            }
            Attachment::Transform(transform) => {
                commands.spawn((
                    transform,
                    sweep_bundle(sweep.stats.clone(), time.elapsed_secs()),
                    DestroyOnSwept
                ));
            },
        }
    }
}

pub fn end_sweep(
    trigger: On<Remove, SweepStage>,
    mut commands: Commands,
    sweeper_q: Query<&Sweeper, With<DestroyOnSwept>>
) {
    if sweeper_q.get(trigger.entity).is_ok() { 
        commands.entity(trigger.entity).despawn();        
    } else {
        commands.entity(trigger.entity).remove::<Sweeper>();
    }
}

#[cfg(feature = "2d")]
pub fn ray_sweep_2d(
    mut sweeper_q: Query<(&mut SweepStage, &Sweeper, &Transform, Entity)>,
    mut attack_w: MessageWriter<HitMessage>,
    mut commands: Commands,
    hit_q: Query<&Transform, Without<Sweeper>>,
    spatial_q: avian2d::spatial_query::SpatialQuery,
    time: Res<Time>,
) {
    let filter = avian2d::spatial_query::SpatialQueryFilter::default();
    // TODO whether sweeps can be interrupted by hit count, or hit multiple targets
    for (
        mut stage, 
        sweeper, 
        sweep_transform,
        sweeper_entity
    ) in &mut sweeper_q {
        if stage.time_next_ray > time.elapsed_secs() { continue };
        info!("Do 1 sweep ray");
        stage.time_next_ray += sweeper.0.interval;
        if stage.stage == sweeper.0.stages - 1 {
            commands.entity(sweeper_entity).remove::<SweepStage>();
        }
        let offset_rot = sweep_transform.rotation * (sweeper.0.formula)(stage.stage);
        let rot_vec = offset_rot * Vec3::Y;
        let direction = Dir2::from_xy_unchecked(rot_vec.x, rot_vec.y);
        let sweep_pos = sweep_transform.translation.truncate();
        if let Some(hit) = spatial_q.cast_ray(
            sweep_pos,
            direction,
            sweeper.0.range,
            true,
            &filter
        ) {
            let hit_transform = hit_q.get(hit.entity).expect("Hit entity has no transform?");
            let diff = hit_transform.translation.truncate() - sweep_pos;
            attack_w.write(HitMessage {
                target: hit.entity,
                source: Some(sweeper_entity),
                distance: diff.length()
            });
        }
        stage.stage += 1;
    }
}

#[cfg(feature = "3d")]
pub fn ray_sweep_3d(
    mut sweeper_q: Query<(&mut SweepStage, &Sweeper, &Transform, Entity)>,
    mut attack_w: MessageWriter<HitMessage>,
    mut commands: Commands,
    hit_q: Query<&Transform, Without<Sweeper>>,
    spatial_q: avian3d::spatial_query::SpatialQuery,
    time: Res<Time>,
) {
    let filter = avian3d::spatial_query::SpatialQueryFilter::default();
    for (
        mut stage, 
        sweeper, 
        sweep_transform,
        sweeper_entity
    ) in &mut sweeper_q {
        if stage.time_next_ray > time.elapsed_secs() { continue };
        stage.time_next_ray += sweeper.0.interval;
        if stage.stage == sweeper.0.stages - 1 {
            commands.entity(sweeper_entity).remove::<SweepStage>();
        }
        let offset_rot = sweep_transform.rotation * (sweeper.0.formula)(stage.stage);
        let rot_vec = offset_rot * Vec3::Y;
        let direction = Dir3::from_xyz_unchecked(rot_vec.x, rot_vec.y, rot_vec.z);
        let sweep_pos = sweep_transform.translation;
        if let Some(hit) = spatial_q.cast_ray(
            sweep_pos,
            direction,
            sweeper.0.range,
            true,
            &filter
        ) {
            let hit_transform = hit_q.get(hit.entity).expect("Hit entity has no transform?");
            let diff = hit_transform.translation - sweep_pos;
            attack_w.write(HitMessage {
                target: hit.entity,
                source: Some(sweeper_entity),
                distance: diff.length()
            });
        }
        stage.stage += 1;
    }
}