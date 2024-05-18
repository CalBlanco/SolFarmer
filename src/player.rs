use bevy::prelude::*;

use crate::{game, map::{self, TileBundle, TileState}};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Mouse;

const SPAWN_X: i32 = 20;
const SPAWN_Y: i32 = 20;
const MOVE_SPEED: f32 = 90.;

#[derive(Component)]
pub struct PlayerAttack (pub Timer);

const ATTACK_COOLDOWN: f32 = 0.4;

#[derive(Component)]
pub struct Hoe;

pub fn setup(mut commands: Commands, assets: Res<AssetServer>){
    let spawn = map::get_world(SPAWN_X, SPAWN_Y);

    commands.spawn( (
        SpriteBundle {
            texture: assets.load("entity/ed.png"),
            transform: Transform::from_xyz(spawn.0, spawn.1, 2.),
            ..default()
        },
        Player,
        PlayerAttack(Timer::from_seconds(ATTACK_COOLDOWN, TimerMode::Once))
    )).with_children(|parent| {
        parent.spawn((SpriteBundle {
            texture: assets.load("images/hoe1.png"),
            transform: Transform::from_xyz(-8., 8., 5.),
            ..default()
        },
        Hoe,
    ));
    });

    commands.spawn( (
        SpriteBundle {
            texture: assets.load("tiles/highlight.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        Mouse
    ));
}

/// Move the player around 
pub fn player_input(
    mut query: Query<(Entity, &mut Transform, &mut PlayerAttack), With<Player>>,
    keycode: Res<ButtonInput<KeyCode>>,
    mouse: Res<game::MyWorldCoords>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>
)
{
    if let Ok((_e, mut transform, mut player_attack)) = query.get_single_mut() {
        // Tick the attack timer
        player_attack.0.tick(time.delta());
        let move_distance = MOVE_SPEED * time.delta_seconds();

        if keycode.pressed(KeyCode::KeyW) {
            transform.translation.y += move_distance;
        }
        if keycode.pressed(KeyCode::KeyS) {
            transform.translation.y -= move_distance;
        }
        if keycode.pressed(KeyCode::KeyD) {
            transform.translation.x += move_distance;
        }
        if keycode.pressed(KeyCode::KeyA) {
            transform.translation.x -= move_distance;
        }

        // If the player can attack and is trying to attack
        if (keycode.pressed(KeyCode::Space)) && (player_attack.0.finished()) {
            // Reset the attack timer
            player_attack.0.reset();
        }

        // Handle the placing/removal of the plant units
        if mouse_input.just_pressed(MouseButton::Left) {
            // Get the indices of the tile clicked on
            let (x, y) = map::get_tile(mouse.0.x, mouse.0.y);
            println!("{}, {}", x, y);
        }
        if mouse_input.just_pressed(MouseButton::Right) {
            
        }
    } 
}

pub fn render_tile_highlight(
    mouse: Res<game::MyWorldCoords>,
    mut tile_highlight: Query<&mut Transform, With<Mouse>>
)
{
    if let Ok(mut transform) = tile_highlight.get_single_mut() {
        let (x, y) = map::get_tile(mouse.0.x, mouse.0.y);
        let (x, y) = map::get_world(x, y);

        transform.translation.x = x;
        transform.translation.y = y;
    }
}

pub fn hoe_swing(
    player_query: Query<&PlayerAttack, With<Player>>,
    mut hoe_query: Query<&mut Transform, With<Hoe>>,
) {
    if let Ok(player_attack) = player_query.get_single() {
        // Are we attacking currently?
        if !player_attack.0.finished() {
            for mut hoe_transform in hoe_query.iter_mut() {
                // Calculate the progress of the swing
                let elapsed = player_attack.0.elapsed().as_secs_f32();
                let duration = player_attack.0.duration().as_secs_f32();
                let swing_progress = elapsed / duration;
                // Use the progress of the swing to determine the angle and the offset
                let angle = (swing_progress * 360.).to_radians();
                let offset_x = 16. * (angle + std::f32::consts::PI).cos();
                let offset_y = 16. * (angle + std::f32::consts::PI).sin();
                // Move the hoe in a circle around the player
                hoe_transform.translation.x = offset_x;
                hoe_transform.translation.y = offset_y;
                // Rotate the hoe while moving it to make it look like a full swing
                hoe_transform.rotation = Quat::from_rotation_z(angle + (0.5 * std::f32::consts::PI));
            }
        } else {
            for mut hoe_transform in hoe_query.iter_mut() {
                // Calculate the offsets using an angle of 0 degrees
                let angle = 0 as f32;
                let offset_x = 16. * (angle + std::f32::consts::PI).cos();
                let offset_y = 16. * (angle + std::f32::consts::PI).sin();
                // Keep the hoe at default position
                hoe_transform.translation.x = offset_x;
                hoe_transform.translation.y = offset_y;
                // Keep the hoe at default rotation
                hoe_transform.rotation = Quat::from_rotation_z(angle);
            }
        }
    }
}

