use bevy::prelude::*;

use crate::{melee::components::*, messages::*};

pub fn start_melee(
    mut attack_r: MessageReader<AttackMessage>,
    mut commands: Commands,
    meleer_q: Query<(&Meleer, Entity), Without<MeleeStage>>,
    time: Res<Time>,
) {
    for attack in attack_r.read() {
        if let Ok(( 
            meleer, 
            meleer_entity
        )) = meleer_q.get(attack.source) {
            match meleer.melee_type {
                Melee::RaySweep { .. } => {
                    commands.entity(meleer_entity).insert(MeleeStage {
                        stage: 0,
                        time_next_ray: time.delta_secs()
                    });
                },
                Melee::Radius { line_of_sight } => { 

                }
            }
        } else {
            continue 
        }
    }
}

#[cfg(feature = "2d")]
pub fn ray_sweep_2d(
    mut meleer_q: Query<(&mut MeleeStage, &Meleer, &Transform, Entity)>,
    mut attack_w: MessageWriter<HitMessage>,
    mut commands: Commands,
    spatial_q: avian2d::spatial_query::SpatialQuery,
    time: Res<Time>,
) {
    let filter = avian2d::spatial_query::SpatialQueryFilter::default();
    for (
        mut stage, 
        meleer, 
        melee_transform,
        melee_entity
    ) in &mut meleer_q {
        if stage.time_next_ray > time.elapsed_secs() { continue };
        info!("Stage: {}", stage.stage);
        let Melee::RaySweep {count, interval, formula}  = meleer.melee_type else { continue };
        stage.time_next_ray += interval;
        if stage.stage == count - 1 {
            commands.entity(melee_entity).remove::<MeleeStage>();
        }
        let offset_rot = melee_transform.rotation * (formula)(stage.stage);
        info!("Offset_rot: {}", offset_rot);
        let rot_vec = offset_rot * Vec3::Y;
        let direction = Dir2::from_xy_unchecked(rot_vec.x, rot_vec.y);
        info!("Direction: {}", direction);
        if let Some(hit) = spatial_q.cast_ray(
            melee_transform.translation.truncate(),
            direction,
            meleer.range,
            true,
            &filter
        ) {
            attack_w.write(HitMessage {
                target: hit.entity,
                source: melee_entity
            });
        }
        stage.stage += 1;
    }
}

#[cfg(feature = "3d")]
pub fn ray_sweep_3d(
    mut meleer_q: Query<(&mut MeleeStage, &Meleer, &Transform, Entity)>,
    mut attack_w: MessageWriter<HitMessage>,
    mut commands: Commands,
    spatial_q: avian3d::spatial_query::SpatialQuery,
    time: Res<Time>,
) {
    let filter = avian3d::spatial_query::SpatialQueryFilter::default();
    for (
        mut stage, 
        meleer, 
        melee_transform,
        melee_entity
    ) in &mut meleer_q {
        if stage.time_next_ray > time.elapsed_secs() { continue };
        let direction = Dir3::new_unchecked( melee_transform.rotation * Vec3::Z);
        if let Some(hit) = spatial_q.cast_ray(
            melee_transform.translation,
            direction,
            meleer.range,
            true,
            &filter
        ) {
            attack_w.write(HitMessage {
                target: hit.entity,
                source: melee_entity
            });
        }
        if let Melee::RaySweep { count, interval, formula } = meleer.melee_type {
            stage.stage += 1;
            stage.time_next_ray += interval;
            if stage.stage == count {
                commands.entity(melee_entity).remove::<MeleeStage>();
            }
        }
    }
}