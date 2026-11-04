#[cfg(test)]
mod tests {
    use std::time::Duration;
    use attack_plugin_core::{melee::{components::*, messages::*}, messages::*, plugin::AttackPlugin, ranged::components::*};
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
        let shooter = app.world_mut().spawn((
            Name::new("Shooter"),
            Shooter {
                projectile_speed: 1.
            },
            Transform::default()
        )).id();
        app.world_mut().write_message(AttackMessage {
            source: shooter
        });
        step(&mut app);
        let exists = app.world().iter_entities().any(|e| e.contains::<Projectile>());
        assert_eq!(exists, true);
    }

    #[test]
    fn step_projectile_test() {
        let mut app = build_app();
        let shooter = app.world_mut().spawn((
            Name::new("Shooter"),
            Shooter {
                projectile_speed: 1.
            },
            Transform::default()
        )).id();
        app.world_mut().write_message(AttackMessage {
            source: shooter
        });
        step(&mut app);
        let mut query = app.world_mut().query::<(&Transform, &Projectile)>();
        let projectile_transform = query.single(app.world()).unwrap().0;
        assert_ne!(projectile_transform.translation, Vec3::ZERO);
    }

    #[test]
    fn projectile_distance_test() {
        let mut app = build_app();
        let shooter = app.world_mut().spawn((
            Name::new("Shooter"),
            Shooter {
                projectile_speed: 1.
            },
            Transform::default()
        )).id();
        app.world_mut().write_message(AttackMessage {
            source: shooter
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
            last_distance = projectile_distance;
        }
    }

     #[test]
    fn projectile_hit_test() {
        let mut app = build_app();
        let shooter = app.world_mut().spawn((
            Name::new("Shooter"),
            Shooter {
                projectile_speed: 50.
            },
            Transform::default()
        )).id();
        app.world_mut().spawn((
            Name::new("Hitbox"),
            Collider::circle(1.),
            Transform::from_translation(Vec3::new(0., 2., 0.))
        ));
        app.world_mut().write_message(AttackMessage {
            source: shooter
        });
        // Hitbox is far enough for 3 frames
        step(&mut app);
        step(&mut app);
        step(&mut app);
        let hit_msgs = app.world().resource::<Messages<HitMessage>>().len();
        assert_eq!(hit_msgs, 1);
        let exists = app.world().iter_entities().any(|e| e.contains::<Projectile>());
        assert_eq!(exists, false);
    }

    #[test]
    fn begin_sweep_test() {
        let mut app = build_app();
        let meleer = app.world_mut().spawn((
            Name::new("Meleer"),
            Meleer {
                melee_type: Melee::RaySweep { count: 3, interval: 0.015, formula: test_sweep },
                range: 3.,
            },
            Transform::default()
        )).id();
        app.world_mut().write_message(AttackMessage {
            source: meleer
        });
        step(&mut app);
        let exists = app.world().iter_entities().any(|e| e.contains::<MeleeStage>());
        assert_eq!(exists, true);
    }

    #[test]
    fn sweep_stage_test() {
        let mut app = build_app();
        let meleer = app.world_mut().spawn((
            Name::new("Meleer"),
            Meleer {
                melee_type: Melee::RaySweep { count: 3, interval: 0.0015, formula: test_sweep },
                range: 3.,
            },
            Transform::default()
        )).id();
        app.world_mut().write_message(AttackMessage {
            source: meleer
        });
        step(&mut app);
        let mut query = app.world_mut().query::<&MeleeStage>();
        {
            let melee_stage = query.single(app.world()).unwrap();
            assert_eq!(melee_stage.stage, 1);
        }
        step(&mut app);
        let mut query = app.world_mut().query::<&MeleeStage>();
        {
            let melee_stage = query.single(app.world()).unwrap();
            assert_eq!(melee_stage.stage, 2);
        }
        step(&mut app);
        let exists = app.world().iter_entities().any(|e| e.contains::<MeleeStage>());
        assert_eq!(exists, false);
    }

    #[test]
    fn sweep_hit_test() {
        let mut app = build_app();
        let meleer = app.world_mut().spawn((
            Name::new("Meleer"),
            Meleer {
                melee_type: Melee::RaySweep { count: 3, interval: 0.0015, formula: test_sweep },
                range: 3.,
            },
            Transform::default()
        )).id();
        app.world_mut().spawn((
            Name::new("Hitbox"),
            Transform::from_translation(Vec3::new(0., 1., 0.)),
            Collider::circle(1.)
        ));
        app.world_mut().write_message(AttackMessage {
            source: meleer
        });
        step(&mut app);
        let hit_msgs = app.world().resource::<Messages<HitMessage>>().len();
        assert_eq!(hit_msgs, 1);
    }

    #[test]
    fn sweep_angle_test() {
        let mut app = build_app();
        let meleer = app.world_mut().spawn((
            Name::new("Meleer"),
            Meleer {
                melee_type: Melee::RaySweep { count: 3, interval: 0.0015, formula: test_sweep },
                range: 5.,
            },
            Transform::default()
        )).id();
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
        app.world_mut().write_message(AttackMessage {
            source: meleer
        });
        step(&mut app);
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
        let meleer = app.world_mut().spawn((
            Name::new("Meleer"),
            Meleer {
                melee_type: Melee::Radius { line_of_sight: false },
                range: 3.,
            },
            Transform::default()
        )).id();
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
        app.world_mut().write_message(AttackMessage {
            source: meleer
        });
        step(&mut app);
        let num_msgs = app.world().resource::<Messages<HitMessage>>().len();
        assert_eq!(num_msgs, 1); // Hitbox 2 should be out of range, only 1 hit message
    }
}