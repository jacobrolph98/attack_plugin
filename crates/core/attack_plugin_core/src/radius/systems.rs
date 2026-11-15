use bevy::prelude::*;
use crate::messages::*;
use crate::radius::messages::*;

#[cfg(feature = "2d")]
pub fn radius_check_2d(
    mut radius_r: MessageReader<RadiusCheckMessage>,
    mut attack_w: MessageWriter<HitMessage>,
    transform_q: Query<&Transform>,
    spatial_q: avian2d::spatial_query::SpatialQuery,
) {
    let filter = avian2d::spatial_query::SpatialQueryFilter::default();

    for check in radius_r.read() {
        let within_bounds = spatial_q.shape_intersections(
            &avian2d::collision::collider::Collider::circle(check.radius), 
            check.pos.truncate(), 
            0., 
            &filter
        );
        let mut hit_msgs: Vec<HitMessage>  = vec![];
        for hit in &within_bounds {
            let Ok(hit_transform) = transform_q.get(*hit) else { 
                info!("Skipping this hit entity because it somehow has no transform?");
                continue
            };
            let diff = hit_transform.translation.truncate() - check.pos.truncate();
            let distance = diff.length();
            if check.line_of_sight {
                // TODO
            }
            hit_msgs.push(HitMessage {
                target: *hit,
                source: check.source,
                distance
            });
        }
        attack_w.write_batch(hit_msgs);
    }
}

#[cfg(feature = "3d")]
pub fn radius_check_3d(
    mut radius_r: MessageReader<RadiusCheckMessage>,
    mut attack_w: MessageWriter<HitMessage>,
    transform_q: Query<&Transform>,
    spatial_q: avian3d::spatial_query::SpatialQuery,
) {
    let filter = avian3d::spatial_query::SpatialQueryFilter::default();
    for check in radius_r.read() {
        let within_bounds = spatial_q.shape_intersections(
            &avian3d::collision::collider::Collider::sphere(check.radius), 
            check.pos, 
            Quat::default(), 
            &filter
        );
        let mut hit_msgs: Vec<HitMessage>  = vec![];
        for hit in &within_bounds {
            let Ok(hit_transform) = transform_q.get(*hit) else { 
                info!("Skipping this hit entity because it somehow has no transform?");
                continue
            };
            let diff = hit_transform.translation - check.pos;
            let distance = diff.length();
            if check.line_of_sight {

            }
            hit_msgs.push(HitMessage {
                target: *hit,
                source: check.source,
                distance
            });
        }
        attack_w.write_batch(hit_msgs);
    }
}