To use this library you should add either attack_plugin_2d or attack_plugin_3d as a dependency. 
This library implements means of processing various kinds of attacks. 
Each attack writes a 'HitMessage' on contact which the developer should read.
For each attack and hit, 'source' should point to the entity making the attack, so the developer can determine what the hit should do.

### Radius
Given a Position, Radius & whether to consider line of sight;
Checks for colliders intersecting with a circle/sphere.
```
radius_msg.write(RadiusCheckMessage {
    pos: Vec3::ZERO,
    radius: 3.,
    line_of_sight: false, // Not yet implemented
    source: None // Should point to entity who's components should dictate how to react to a hit
});
```

### Sweep
Given an 'Attachment' (Transform, or existing entity with transform), a range, interval, ray_count and formula;
Casts 'stages' amount of rays with a given interval, at an offset angle according to the provided formula and stage.
Can be used to create a melee system akin to Left 4 Dead 2.
```
/// Example function to use as a formula, providing an angle offset at different stages. 
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

sweep_msg.write(StartSweep {
    attach: Attachment::Transform(Transform::default()),
    stats: Sweep {
        stages: 3,
        interval: 0.015,
        range: 5.,
        formula: test_sweep,
    },
    source: None
});
```

### Projectile 
Given a Position, Direction, Speed, optional position to track;
Spawns a projectile that follows the given trajectory (or may track a given position with a max_angle per second)
```
projectile_msg.write(SpawnProjectile {
    pos: Vec3::ZERO,
    direction: Quat::default(),
    speed: 1.,
    track: None,
    source: None
});
```
Example providing a position & max degrees/s to turn to track a target.
```
projectile_msg.write(SpawnProjectile {
    pos: Vec3::ZERO,
    direction: Quat::default(),
    speed: 1.,
    track: Some((Vec3::splat(10.), 90.)),
    source: None
});
```