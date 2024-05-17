use bevy::prelude::*;

use crate::map;

use super::{RESOLUTION_X, RESOLUTION_Y};

#[derive(Component)]
pub struct Player;

const SPAWN_X: i32 = 20;
const SPAWN_Y: i32 = 20;
const MOVE_SPEED: f32 = 90.;


pub fn setup(mut commands: Commands, assets: Res<AssetServer>){
    let spawn = map::get_world(SPAWN_X, SPAWN_Y);

    commands.spawn( (
        SpriteBundle {
            texture: assets.load("entity/ed.png"),
            transform: Transform::from_xyz(spawn.0, spawn.1, 1.),
            ..default()
        },
        Player
    ));
}

/// Move the player around 
pub fn player_movement(
    mut query: Query<(Entity, &mut Transform), With<Player>>,
    keycode: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
)
{
    if let Ok((e, mut transform)) = query.get_single_mut() {

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

    } 
}