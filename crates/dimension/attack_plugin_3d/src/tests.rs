#[cfg(test)]
mod tests {
    use std::time::Duration;

use attack_plugin_core::{messages::*, plugin::AttackPlugin, ranged::components::*};
    use avian3d::prelude::*;
    use bevy::{log::LogPlugin, mesh::MeshPlugin, prelude::*, time::TimeUpdateStrategy};
    
    fn build_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, LogPlugin::default(), AssetPlugin::default(), MeshPlugin));
        app.add_plugins(PhysicsPlugins::default());
        app.add_plugins(AttackPlugin);
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(16)));
        app.finish();
        app.cleanup();
        app
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
    fn hitbox_test() {
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
            Collider::sphere(1.),
            Transform::from_translation(Vec3::new(0., 0., 2.))
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
}