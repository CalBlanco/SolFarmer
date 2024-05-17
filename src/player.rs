use bevy::prelude::*;

use crate::{game, map};

use super::{RESOLUTION_X, RESOLUTION_Y};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Mouse;

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

pub fn render_tile_highlight(
    mouse: Res<game::MyWorldCoords>,
    mut tile_highlight: Query<(&mut Transform), With<Mouse>>
)
{
    if let Ok(mut transform) = tile_highlight.get_single_mut() {
        let (x, y) = map::get_tile(mouse.0.x, mouse.0.y);
        let (x, y) = map::get_world(x, y);

        transform.translation.x = x;
        transform.translation.y = y;
    }
}