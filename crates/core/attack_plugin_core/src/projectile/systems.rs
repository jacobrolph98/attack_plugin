//use attack_plugin_math::{apply_offset, forward_direction, origin_from_translation};
use bevy::prelude::*;

use crate::{messages::*, projectile::{components::*, messages::*}};

pub fn spawn_projectile(
    mut spawn_r: MessageReader<SpawnProjectile>,
    mut commands: Commands,
) {
    for shot in spawn_r.read() {
        let projectile = commands.spawn((
            Projectile {
                source: shot.source
            },
            ProjectileSpeed(shot.speed),
            ProjectileDistance(0.),
            Transform::from_translation(shot.pos).with_rotation(shot.direction)
        )).id();
        if let Some((track_pos, turn_radius)) = shot.track {
            commands.entity(projectile).insert((
                ProjectileTrackPosition(track_pos),
                ProjectileTurnRate(turn_radius)
            ));
        }
    }
}

pub fn rotate_projectile(
    mut projectile_q: Query<(&mut Transform, &ProjectileTurnRate, &ProjectileTrackPosition)>,
    time: Res<Time>,
) {
    for (mut transform, turn_radius, target_pos) in &mut projectile_q {
        let dir = target_pos.0 - transform.translation;

        //let target_rot = Transform::from_translation(transform.translation).looking_at(target_pos.0, Vec3::Y).rotation;

        let target_rot = Quat::from_rotation_arc(Vec3::Z, dir.normalize());
        let max_rot = turn_radius.0.to_radians() * time.delta_secs();
        transform.rotation = transform.rotation.rotate_towards(target_rot, max_rot);
    }
}

#[cfg(feature = "2d")]
pub fn step_projectile_2d(
    mut projectile_q: Query<(&mut Transform, &mut ProjectileDistance, &ProjectileSpeed, Entity)>,
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
                source: Some(projectile_entity),
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
    mut projectile_q: Query<(&mut Transform, &mut ProjectileDistance, &ProjectileSpeed, Entity)>,
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
                source: Some(projectile_entity),
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