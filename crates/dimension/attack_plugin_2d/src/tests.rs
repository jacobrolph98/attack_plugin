#[cfg(test)]
mod tests {
    use std::time::Duration;
    use attack_plugin_core::{melee::components::*, messages::*, plugin::AttackPlugin, ranged::components::*};
    use avian2d::prelude::*;
    use bevy::{log::LogPlugin, prelude::*, time::TimeUpdateStrategy};
    
    fn build_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, LogPlugin::default()));
        app.add_plugins((PhysicsPlugins::default(), AttackPlugin));
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(16)));
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
        app.update();
        app.update();
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
        app.update();
        app.update();
        let mut query = app.world_mut().query::<(&Transform, &Projectile)>();
        let projectile_transform = query.single(app.world()).unwrap().0;
        assert_ne!(projectile_transform.translation, Vec3::ZERO);
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
        app.update();
        app.update();
        app.update();
        let hit_empty = app.world().resource::<Messages<HitMessage>>().is_empty();
        assert_eq!(hit_empty, false);
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
        app.update();
        app.update();
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
        app.update();
        app.update();
        let mut query = app.world_mut().query::<&MeleeStage>();
        {
            let melee_stage = query.single(app.world()).unwrap();
            assert_eq!(melee_stage.stage, 1);
        }
        app.update();
        let mut query = app.world_mut().query::<&MeleeStage>();
        {
            let melee_stage = query.single(app.world()).unwrap();
            assert_eq!(melee_stage.stage, 2);
        }
        app.update();
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
            Transform::from_translation(Vec3::new(0., 0., 2.)),
            Collider::circle(1.)
        ));
        app.world_mut().write_message(AttackMessage {
            source: meleer
        });
        app.update();
        app.update();
        let hit_empty = app.world().resource::<Messages<HitMessage>>().is_empty();
        assert_eq!(hit_empty, false);
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
        app.update();
        app.update();
        let hits = app.world().resource::<Messages<HitMessage>>();
        assert_eq!(hits.is_empty(), false);
        app.update();
        let hits = app.world().resource::<Messages<HitMessage>>();
        assert_eq!(hits.is_empty(), false);
        app.update();
        let hits = app.world().resource::<Messages<HitMessage>>();
        assert_eq!(hits.is_empty(), false);
        app.update();

    }
}