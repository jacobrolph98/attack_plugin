//use attack_plugin_math::{apply_offset, forward_direction, origin_from_translation};
use bevy::prelude::*;

use crate::{messages::*, ranged::components::*};

pub fn spawn_projectile(
    mut attack_r: MessageReader<AttackMessage>,
    mut commands: Commands,
    shooter_q: Query<(&Transform, &Shooter, Entity)>,
) {
    for attack in attack_r.read() {
        if let Ok((
            shooter_transform, 
            shooter, 
            shooter_entity
        )) = shooter_q.get(attack.source) {
            commands.spawn((
                Projectile {
                    source: shooter_entity
                },
                ProjectileSpeed(shooter.projectile_speed),
                ProjectileDistance(0.),
                shooter_transform.clone()
            ));
        } else {
            continue 
        }
    }
}

#[cfg(feature = "2d")]
pub fn step_projectile_2d(
    mut projectile_q: Query<(&mut Transform, &mut ProjectileDistance, &ProjectileSpeed, &Projectile, Entity)>,
    mut attack_w: MessageWriter<HitMessage>,
    mut commands: Commands,
    spatial_q: avian2d::spatial_query::SpatialQuery,
    time: Res<Time>,
) {
    let filter = avian2d::spatial_query::SpatialQueryFilter::default();
    for (
        mut transform, 
        mut proj_distance,
        speed, 
        projectile,
        projectile_entity
    ) in &mut projectile_q {
        let distance = speed.0 * time.delta_secs();
        let origin = transform.translation.truncate();
        let rot_vec = transform.rotation * Vec3::Y;
        let direction = Dir2::from_xy_unchecked(rot_vec.x, rot_vec.y);
        /* TODO FOR USE WHEN PENETRATION IMPLEMENTED
        let hits = spatial_q.ray_hits(
            transform.translation.truncate(),
            direction,
            distance,
            5, // Revise to account for speed/distance of projectile this frame
            true,
            &filter
        ).sort_unstable_by(|a, b| a.distance.total_cmp(&b.distance));
            */
        if let Some(hit) = spatial_q.cast_ray(
            origin,
            direction,
            distance,
            true,
            &filter
        ) {
            attack_w.write(HitMessage {
                target: hit.entity,
                source: projectile.source,
                distance: proj_distance.0
            });
            commands.entity(projectile_entity).despawn();
        } else {
            let offset = direction * distance;
            transform.translation += Vec3::new(offset.x, offset.y, 0.);
            proj_distance.0 += distance;
        }
    }
}

#[cfg(feature = "3d")]
pub fn step_projectile_3d(
    mut projectile_q: Query<(&mut Transform, &mut ProjectileDistance, &ProjectileSpeed, &Projectile, Entity)>,
    mut attack_w: MessageWriter<HitMessage>,
    mut commands: Commands,
    spatial_q: avian3d::spatial_query::SpatialQuery,
    time: Res<Time>,
) {
    let filter = avian3d::spatial_query::SpatialQueryFilter::default();
    for (
        mut transform, 
        mut proj_distance,
        speed, 
        projectile,
        projectile_entity
    ) in &mut projectile_q {
        let distance = speed.0 * time.delta_secs();
        let direction = Dir3::new_unchecked( transform.rotation * Vec3::Z);
        /* TODO FOR USE WHEN PENETRATION IMPLEMENTED
        let hits = spatial_q.ray_hits(
            transform.translation.truncate(),
            direction,
            distance,
            5, // Revise to account for speed/distance of projectile this frame
            true,
            &filter
        ).sort_unstable_by(|a, b| a.distance.total_cmp(&b.distance));
            */
        if let Some(hit) = spatial_q.cast_ray(
            transform.translation,
            direction,
            distance,
            true,
            &filter
        ) {
            attack_w.write(HitMessage {
                target: hit.entity,
                source: projectile.source,
                distance: proj_distance.0
            });
            commands.entity(projectile_entity).despawn();
        } else {
            let offset = direction * distance;
            transform.translation += Vec3::new(offset.x, offset.y, offset.z);
            proj_distance.0 += distance;
        }
    }
}