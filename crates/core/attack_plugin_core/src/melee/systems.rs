use bevy::prelude::*;
use crate::{melee::{components::*, messages::MeleeRadiusMessage}, messages::*};

pub fn start_melee(
    mut attack_r: MessageReader<AttackMessage>,
    mut radius_w: MessageWriter<MeleeRadiusMessage>,
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
                    radius_w.write(MeleeRadiusMessage {
                        entity: meleer_entity,
                        line_of_sight,
                    });
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
    hit_q: Query<&Transform, Without<Meleer>>,
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
        let Melee::RaySweep {count, interval, formula}  = meleer.melee_type else { continue };
        stage.time_next_ray += interval;
        if stage.stage == count - 1 {
            commands.entity(melee_entity).remove::<MeleeStage>();
        }
        let offset_rot = melee_transform.rotation * (formula)(stage.stage);
        let rot_vec = offset_rot * Vec3::Y;
        let direction = Dir2::from_xy_unchecked(rot_vec.x, rot_vec.y);
        let melee_pos = melee_transform.translation.truncate();
        if let Some(hit) = spatial_q.cast_ray(
            melee_pos,
            direction,
            meleer.range,
            true,
            &filter
        ) {
            let hit_transform = hit_q.get(hit.entity).expect("Hit entity has no transform?");
            let diff = hit_transform.translation.truncate() - melee_pos;
            attack_w.write(HitMessage {
                target: hit.entity,
                source: melee_entity,
                distance: diff.length()
            });
        }
        stage.stage += 1;
    }
}

#[cfg(feature = "2d")]
pub fn radius_check_2d(
    mut attack_w: MessageWriter<HitMessage>,
    meleer_q: Query<(&Meleer, &Transform, Entity)>,
    hit_q: Query<&Transform, Without<Meleer>>,
    spatial_q: avian2d::spatial_query::SpatialQuery,
) {
    let filter = avian2d::spatial_query::SpatialQueryFilter::default();
    for (
        meleer, 
        melee_transform,
        melee_entity
    ) in &meleer_q {
        let within_bounds = spatial_q.shape_intersections(
            &avian2d::collision::collider::Collider::circle(meleer.range), 
            melee_transform.translation.truncate(), 
            0., 
            &filter
        );
        let Melee::Radius { line_of_sight} = meleer.melee_type else { continue };
        let mut hit_msgs: Vec<HitMessage> = vec![];
        for hit in &within_bounds {
            let hit_transform = hit_q.get(*hit).expect("Hit entity has no transform?");
            let diff = hit_transform.translation.truncate() - melee_transform.translation.truncate();
            let distance = diff.length();
            if line_of_sight {
                // TODO figure out appropriate filter and skip if obstructed
                let hit = spatial_q.cast_ray(
                    melee_transform.translation.truncate(),
                    Dir2::new(diff).expect("Meleer & hit have same translation?"),
                    distance,
                    true,
                    &filter
                );
            }
            hit_msgs.push(HitMessage {
                    target: *hit,
                    source: melee_entity,
                    distance
                });
        }
        attack_w.write_batch(hit_msgs);
    }
}

#[cfg(feature = "3d")]
pub fn ray_sweep_3d(
    mut meleer_q: Query<(&mut MeleeStage, &Meleer, &Transform, Entity)>,
    mut attack_w: MessageWriter<HitMessage>,
    mut commands: Commands,
    hit_q: Query<&Transform, Without<Meleer>>,
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
        let Melee::RaySweep {count, interval, formula}  = meleer.melee_type else { continue };
        stage.time_next_ray += interval;
        if stage.stage == count - 1 {
            commands.entity(melee_entity).remove::<MeleeStage>();
        }
        let offset_rot = melee_transform.rotation * (formula)(stage.stage);
        let rot_vec = offset_rot * Vec3::Y;
        let direction = Dir3::from_xyz_unchecked(rot_vec.x, rot_vec.y, rot_vec.z);
        if let Some(hit) = spatial_q.cast_ray(
            melee_transform.translation,
            direction,
            meleer.range,
            true,
            &filter
        ) {
            let hit_transform = hit_q.get(hit.entity).expect("Hit entity has no transform?");
            let diff = hit_transform.translation - melee_transform.translation;
            attack_w.write(HitMessage {
                target: hit.entity,
                source: melee_entity,
                distance: diff.length()
            });
        }
        stage.stage += 1;        
    }
}

#[cfg(feature = "3d")]
pub fn radius_check_3d(
    mut attack_w: MessageWriter<HitMessage>,
    meleer_q: Query<(&Meleer, &Transform, Entity)>,
    hit_q: Query<&Transform, Without<Meleer>>,
    spatial_q: avian3d::spatial_query::SpatialQuery,
) {
    let filter = avian3d::spatial_query::SpatialQueryFilter::default();
    for (
        meleer, 
        melee_transform,
        melee_entity
    ) in &meleer_q {
        let within_bounds = spatial_q.shape_intersections(
            &avian3d::prelude::Collider::sphere(meleer.range), 
            melee_transform.translation, 
            Quat::default(), 
            &filter
        );
        let Melee::Radius { line_of_sight} = meleer.melee_type else { continue };
        let mut hit_msgs: Vec<HitMessage> = vec![];
        for hit in &within_bounds {
            let hit_transform = hit_q.get(*hit).expect("Hit entity has no transform?");
            let diff = hit_transform.translation - melee_transform.translation;
            let distance = diff.length();
            if line_of_sight {
                // TODO figure out appropriate filter and skip if obstructed
                let hit = spatial_q.cast_ray(
                    melee_transform.translation,
                    Dir3::new(diff).expect("Meleer & hit have same translation?"),
                    distance,
                    true,
                    &filter
                );
            }
            hit_msgs.push(HitMessage {
                    target: *hit,
                    source: melee_entity,
                    distance
                });
        }
        attack_w.write_batch(hit_msgs);
    }
}