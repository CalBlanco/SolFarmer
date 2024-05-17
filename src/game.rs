use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, prelude::*, window::PrimaryWindow};

use crate::{map, player};

use super::{AppState, RESOLUTION_X, RESOLUTION_Y};


/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;



pub fn build_plugin(app: &mut App){
    app
    .add_systems(OnEnter(AppState::Game), (
        setup,
        player::setup,
    ))
    .init_resource::<MyWorldCoords>()
    .add_systems(Update, (
        my_cursor_system
    ).run_if(in_state(AppState::Game)))
    .add_systems(FixedUpdate, (
        player::player_movement
    ).run_if(in_state(AppState::Game)))
    .add_systems(OnExit(AppState::Game), cleanup);
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    
    commands.spawn(( //Camera with bloom settings enabled
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz((RESOLUTION_X / 2.) - 16.0, (RESOLUTION_Y / 2.) - 16.0 , 0.),
            ..default()
        },
        BloomSettings{
            intensity: 0.1,
            ..default()
        },
        MainCamera
    ));

    crate::map::draw_background(commands, assets);

    
}


fn cleanup(mut commands: Commands, query: Query<(Entity, &Transform)>,  cams: Query<Entity, With<Camera>>,){
    for (e, _) in query.iter(){
        commands.entity(e).despawn();
    }

    for ent in &cams{
        commands.entity(ent).despawn();
    }
}

/// Get Cursor Position
fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
        //let tile_cords = map::get_tile(world_position.x, world_position.y);
        // eprintln!("World coords: {}/{}  - Tile: {}/{}", world_position.x, world_position.y, tile_cords.0, tile_cords.1);
    }
}
