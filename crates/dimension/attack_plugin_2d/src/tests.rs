#[cfg(test)]
mod tests {
    use std::time::Duration;
    use attack_plugin_core::{
        messages::*, 
        plugin::AttackPlugin, 
        projectile::{components::*, messages::SpawnProjectile}, 
        radius::messages::RadiusCheckMessage, 
        sweep::{components::*, messages::StartSweep, types::{Attachment, Sweep}}
    };
    use avian2d::prelude::*;
    use bevy::{log::LogPlugin, prelude::*};
    
    fn build_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, LogPlugin::default()));
        app.add_plugins((PhysicsPlugins::default(), AttackPlugin));
        app.finish();
        app.cleanup();
        app
    }

    fn test_sweep(stage: usize) -> Quat {
        if stage == 0 {
            let angle = std::f32::consts::FRAC_PI_4;
            Quat::from_rotation_z(angle)
        } else if stage == 1 {
            let angle = 0.0;
            Quat::from_rotation_z(angle)
        } else if stage == 2 {
            let angle = -std::f32::consts::FRAC_PI_4;
            Quat::from_rotation_z(angle)            
        } else {
            Quat::IDENTITY            
        }
    }

    fn step(app: &mut App) {
        let delta = Duration::from_millis(16);
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(delta);
        }
        app.world_mut().run_schedule(FixedUpdate);
    }

    #[test]
    fn spawn_projectile_test() {
        let mut app = build_app();
        app.world_mut().write_message(SpawnProjectile {
            pos: Vec3::ZERO,
            direction: Quat::default(),
            speed: 1.,
            track: None,
            source: None
        });
        step(&mut app);
        let exists = app.world().iter_entities().any(|e| e.contains::<Projectile>());
        assert_eq!(exists, true);
    }

    #[test]
    fn step_projectile_test() {
        let mut app = build_app();
        app.world_mut().write_message(SpawnProjectile {
            pos: Vec3::ZERO,
            direction: Quat::default(),
            speed: 1.,
            track: None,
            source: None
        });
        step(&mut app);
        let mut query = app.world_mut().query::<(&Transform, &Projectile)>();
        let projectile_transform = query.single(app.world()).unwrap().0;
        assert_ne!(projectile_transform.translation, Vec3::ZERO);
    }

    #[test]
    fn projectile_distance_test() {
        let mut app = build_app();
        app.world_mut().write_message(SpawnProjectile {
            pos: Vec3::ZERO,
            direction: Quat::default(),
            speed: 1.,
            track: None,
            source: None
        });
        let mut last_distance = 0.;
        step(&mut app);
        {
            let mut query = app.world_mut().query::<&ProjectileDistance>();
            let projectile_distance = query.single(app.world()).unwrap().0;
            assert_ne!(last_distance, projectile_distance);
            last_distance = projectile_distance;
        }
        step(&mut app);
        {
            let mut query = app.world_mut().query::<&ProjectileDistance>();
            let projectile_distance = query.single(app.world()).unwrap().0;
            assert_ne!(last_distance, projectile_distance);
            last_distance = projectile_distance;
        }
        step(&mut app);
        {
            let mut query = app.world_mut().query::<&ProjectileDistance>();
            let projectile_distance = query.single(app.world()).unwrap().0;
            assert_ne!(last_distance, projectile_distance);
        }
    }

     #[test]
    fn projectile_hit_test() {
        let mut app = build_app();
        app.world_mut().spawn((
            Name::new("Hitbox"),
            Collider::circle(1.),
            Transform::from_translation(Vec3::new(0., 2., 0.))
        ));
        app.world_mut().write_message(SpawnProjectile {
            pos: Vec3::ZERO,
            direction: Quat::default(),
            speed: 50.,
            track: None,
            source: None
        });
        // Hitbox is far enough for 2 frames
        step(&mut app);
        step(&mut app);
        let hit_msgs = app.world().resource::<Messages<HitMessage>>().len();
        assert_eq!(hit_msgs, 1);
        let exists = app.world().iter_entities().any(|e| e.contains::<Projectile>());
        assert_eq!(exists, false);
    }
    
    #[test]
    fn spawn_track_projectile_test() {
        let mut app = build_app();
        app.world_mut().write_message(SpawnProjectile {
            pos: Vec3::ZERO,
            direction: Quat::default(),
            speed: 0.,
            track: Some((Vec3::splat(10.), 90.)),
            source: None
        });
        step(&mut app);
        let mut query = app.world_mut().query_filtered::<Entity, (With<ProjectileTrackPosition>, With<ProjectileTurnRate>)>();
        assert_eq!(query.single(app.world()).is_ok(), true);
    }

    #[test]
    fn projectile_turn_test() {
        let mut app = build_app();
        app.world_mut().write_message(SpawnProjectile {
            pos: Vec3::ZERO,
            direction: Quat::default(),
            speed: 0.,
            track: Some((Vec3::splat(10.), 90.)),
            source: None
        });
        step(&mut app);
        let initial_angle =  {
            let mut query = app.world_mut().query_filtered::<&Transform, With<Projectile>>();
            let proj_trans = query.single(app.world()).unwrap();
            (proj_trans.rotation * Vec3::Z).angle_between((Vec3::splat(10.) - proj_trans.translation).normalize())
        };
        step(&mut app);
        let next_angle =  {
            let mut query = app.world_mut().query_filtered::<&Transform, With<Projectile>>();
            let proj_trans = query.single(app.world()).unwrap();
            (proj_trans.rotation * Vec3::Z).angle_between((Vec3::splat(10.) - proj_trans.translation).normalize())
        };
        assert!(next_angle < initial_angle);
    }

    #[test]
    fn projectile_turn_rate_test() {
        let mut app = build_app();
        let angle_per_sec = 90.;
        app.world_mut().write_message(SpawnProjectile {
            pos: Vec3::ZERO,
            direction: Quat::default(),
            speed: 0.,
            track: Some((Vec3::splat(10.), angle_per_sec)),
            source: None
        });
        step(&mut app);
        let first_rotation =  {
            let mut query = app.world_mut().query_filtered::<&Transform, With<Projectile>>();
            let proj_trans = query.single(app.world()).unwrap();
            proj_trans.rotation
        };
        step(&mut app);
        let second_rotation =  {
            let mut query = app.world_mut().query_filtered::<&Transform, With<Projectile>>();
            let proj_trans = query.single(app.world()).unwrap();
            proj_trans.rotation
        };
        let angle = first_rotation.angle_between(second_rotation);
        let max_angle = angle_per_sec * 0.016;
        info!("real: {}, max: {}", angle, max_angle);
        assert!(angle <= max_angle);
    }

    #[test]
    fn begin_sweep_test() {
        let mut app = build_app();
        let attachee = app.world_mut().spawn((
            Name::new("Attachee"),
            Transform::default()
        )).id();
        app.world_mut().write_message_batch([
            StartSweep {
                attach: Attachment::Entity(attachee),
                stats: Sweep {
                    stages: 3,
                    interval: 0.015,
                    range: 1.,
                    formula: test_sweep,
                },
                source: None
            },
            StartSweep {
                attach: Attachment::Transform(Transform::default()),
                stats: Sweep {
                    stages: 3,
                    interval: 0.015,
                    range: 1.,
                    formula: test_sweep,
                },
                source: None
            }
        ]);
        step(&mut app);
        {
            // Test that sweep components successfuly appended to existing entity
            let mut query = app.world_mut().query::<(&Sweeper, &SweepStage, &Name)>();
            assert_eq!(query.single(app.world()).unwrap().2.as_str(), "Attachee");
        }
        {
            // Test that sweep components successfully created new entity
            let mut query = app.world_mut().query_filtered::<(&Sweeper, &SweepStage), Without<Name>>();
            assert_eq!(query.single(app.world()).is_ok(), true);
        }
    }

    #[test]
    fn end_sweep_test() {
        let mut app = build_app();
        let attachee = app.world_mut().spawn((
            Name::new("Attachee"),
            Transform::default()
        )).id();
        app.world_mut().write_message_batch([
            StartSweep {
                attach: Attachment::Entity(attachee),
                stats: Sweep {
                    stages: 2,
                    interval: 0.015,
                    range: 1.,
                    formula: test_sweep,
                },
                source: None
            },
            StartSweep {
                attach: Attachment::Transform(Transform::default()),
                stats: Sweep {
                    stages: 2,
                    interval: 0.015,
                    range: 1.,
                    formula: test_sweep,
                },
                source: None
            }
        ]);
        step(&mut app);
        let spawned_entity = {
            let mut query = app.world_mut().query_filtered::<Entity, (With<Sweeper>, Without<Name>)>();
            query.single(app.world()).unwrap()
        };
        step(&mut app);
        {
            // Test that sweep components removed themselves but attached entity remains
            let mut query = app.world_mut().query_filtered::<&Name, (Without<Sweeper>, Without<SweepStage>)>();
            assert_eq!(query.single(app.world()).is_ok(), true);
        }
        {
            // Test that sweep components removed themselves and the entity they spawned
            assert_eq!(app.world().get_entity(spawned_entity).is_ok(), false);
        }
    }

    #[test]
    fn sweep_stage_test() {
        let mut app = build_app();
        app.world_mut().write_message(StartSweep {
            attach: Attachment::Transform(Transform::default()),
            stats: Sweep {
                stages: 3,
                interval: 0.015,
                range: 3.,
                formula: test_sweep,
            },
            source: None
        });
        step(&mut app);
        let mut query = app.world_mut().query::<&SweepStage>();
        {
            let stage = query.single(app.world()).unwrap();
            assert_eq!(stage.stage, 1);
        }
        step(&mut app);
        let mut query = app.world_mut().query::<&SweepStage>();
        {
            let stage = query.single(app.world()).unwrap();
            assert_eq!(stage.stage, 2);
        }
        step(&mut app);
        let exists = app.world().iter_entities().any(|e| e.contains::<SweepStage>());
        assert_eq!(exists, false);
    }

    #[test]
    fn sweep_hit_test() {
        let mut app = build_app();
        app.world_mut().spawn((
            Name::new("Hitbox"),
            Transform::from_translation(Vec3::new(0., 1., 0.)),
            Collider::circle(1.)
        ));
        app.world_mut().write_message(StartSweep {
            attach: Attachment::Transform(Transform::default()),
            stats: Sweep {
                stages: 3,
                interval: 0.015,
                range: 3.,
                formula: test_sweep,
            },
            source: None
        });
        step(&mut app);
        let hit_msgs = app.world().resource::<Messages<HitMessage>>().len();
        assert_eq!(hit_msgs, 1);
    }

    #[test]
    fn sweep_angle_test() {
        let mut app = build_app();
        app.world_mut().spawn((
            Name::new("HitboxLeft"),
            Transform::from_translation(Vec3::new(-2., 2., 0.)), // At -45 degree angle, left offset ray from sweep should hit
            Collider::circle(1.)
        ));
        app.world_mut().spawn((
            Name::new("HitboxMid"),
            Transform::from_translation(Vec3::new(0., 2.,0.)), // At 0 degree angle, middle ray from sweep should hit
            Collider::circle(1.)
        ));
        app.world_mut().spawn((
            Name::new("HitboxRight"),
            Transform::from_translation(Vec3::new(2., 2., 0.)), // At 45 degree angle, right offset ray from sweep should hit
            Collider::circle(1.)
        ));
        app.world_mut().write_message(StartSweep {
            attach: Attachment::Transform(Transform::default()),
            stats: Sweep {
                stages: 3,
                interval: 0.015,
                range: 5.,
                formula: test_sweep,
            },
            source: None
        });
        step(&mut app);
        let hits = app.world().resource::<Messages<HitMessage>>();
        assert_eq!(hits.is_empty(), false);
        step(&mut app);
        let hits = app.world().resource::<Messages<HitMessage>>();
        assert_eq!(hits.is_empty(), false);
        step(&mut app);
        let hits = app.world().resource::<Messages<HitMessage>>();
        assert_eq!(hits.is_empty(), false);
        step(&mut app);

    }
    
    #[test]
    fn radius_hit_test() {
        let mut app = build_app();
        app.world_mut().spawn((
            Name::new("Hitbox1"),
            Transform::from_translation(Vec3::new(0., 2., 0.)),
            Collider::circle(1.)
        ));
        app.world_mut().spawn((
            Name::new("Hitbox2"),
            Transform::from_translation(Vec3::new(0., 4.1, 0.)),
            Collider::circle(1.)
        ));        
        app.world_mut().write_message(RadiusCheckMessage {
            pos: Vec3::ZERO,
            radius: 3.,
            line_of_sight: false,
            source: None
        });
        step(&mut app);
        let num_msgs = app.world().resource::<Messages<HitMessage>>().len();
        assert_eq!(num_msgs, 1); // Hitbox 2 should be out of range, only 1 hit message
    }
}