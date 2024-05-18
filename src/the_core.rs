use bevy::prelude::*;

use crate::{game, map::{self, TileBundle, TileState}, spriteanims};

const CORE_ROTATE_PERIOD: f32 = 0.8;

#[derive(Component)]
pub struct Core;

#[derive(Component)]
pub struct CoreRotator;

#[derive(Component)]
pub struct CoreDeath;

#[derive(Component)]
pub struct CoreRotation (pub Timer);

#[derive(Bundle)]
pub struct CoreBundle {
    core:  Core,
    core_rotation: CoreRotation,
}

impl CoreBundle {
    pub fn new() -> CoreBundle {
        CoreBundle {
            core: Core,
            core_rotation: CoreRotation(Timer::from_seconds(CORE_ROTATE_PERIOD, TimerMode::Repeating)),
        }
    }
} 

pub fn setup (mut commands: Commands, assets: Res<AssetServer>) {
    // Spawn the core
    commands.spawn(
        (
            CoreBundle::new(),
            SpriteBundle {
                texture: assets.load("images/core_orb.png"),
                transform: Transform::from_xyz(20. * 32., 2. * 32., 2.),
                ..default()
            }
        )
    ).with_children(|parent| {
        // Add the children
        // Core Base Image (the orb)
        parent.spawn(
            SpriteBundle {
                texture: assets.load("images/core_orb.png"),
                transform: Transform::from_xyz(0., 0., 2.),
                ..default()
            }
        );
        // Spawn the core spin object
        parent.spawn(
            (SpriteBundle {
                texture: assets.load("images/core_spin.png"),
                transform: Transform::from_xyz(0., 0., 3.),
                ..default()
            },
            CoreRotator, 
        ));
        // Spawn the mosaic layer that indicates damage
        parent.spawn(
            SpriteBundle {
                texture: assets.load("images/core_death.png"),
                transform: Transform::from_xyz(0., 0., 4.),
                ..default()
            }
        );
    });
}

pub fn core_update (
    mut core_query: Query<&mut CoreRotation, With<Core>>,
    mut param_set: ParamSet<(
        Query<&mut Transform, With<CoreRotator>>,
        Query<&mut Transform, With<CoreDeath>>,
        Query<&mut Transform, With<Core>>,
    )>,
    time: Res<Time>
) {
    // Get the core rotation value
    if let Ok(mut core_rotation) = core_query.get_single_mut() {
        core_rotation.0.tick(time.delta());
        // Calculate the progress of the swing
        let elapsed = core_rotation.0.elapsed().as_secs_f32();
        let duration = core_rotation.0.duration().as_secs_f32();
        let progress = elapsed / duration;
        // Get the core rotator
        for mut core_rotator_transform in param_set.p0().iter_mut() {
            let angle = (progress * 360.).to_radians();
            // Rotate the hoe while moving it to make it look like a full swing
            core_rotator_transform.rotation = Quat::from_rotation_z(angle + (0.5 * std::f32::consts::PI));
        }

        // Get the core death
        for mut core_death_transform in param_set.p1().iter_mut() {
            
        }

        // Get the core breathing
        for mut core_breathe_transform in param_set.p2().iter_mut() {
            // Calculate a cosine breathe value
            let breath_progress = (progress * 360.).to_radians().cos();
            // Apply breathe progress to orb
            core_breathe_transform.scale = Vec3::new(
                1.0 + (breath_progress * 0.05),
                1.0 + (breath_progress * 0.05),
                1.0,
            );
        }
    }
}