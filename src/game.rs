use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, ecs::query, prelude::*, window::PrimaryWindow};
use bevy::render::*;
use crate::{map::{self, Position}, player};

use super::{AppState, RESOLUTION_X, RESOLUTION_Y};


/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct MyWorldCoords(pub Vec2);

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

    .insert_resource(DayNightCycle::new(DAY_DURATION))
    .add_systems(Update, (
        update_day_night_cycle
    ).run_if(in_state(AppState::Game)))
    

    .add_systems(Update, (
        my_cursor_system
    ).run_if(in_state(AppState::Game)))

    .add_systems(FixedUpdate, (
        player::player_input,
        player::render_tile_highlight,
        player::hoe_swing,
        player::mouse_tile_select,

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

// Constants for day duration (in seconds)
const DAY_DURATION: f32 = 20.0;
const DAY_LIGHT_LEVEL: f32 = 1.5;
const NIGHT_LIGHT_LEVEL: f32 = 0.6;

// Resource to hold the day-night cycle timer
#[derive(Resource)]
struct DayNightCycle {
    timer: Timer,
}

impl DayNightCycle {
    fn new(day_duration: f32) -> Self {
        DayNightCycle {
            timer: Timer::from_seconds(day_duration, TimerMode::Repeating),
        }
    }
}

fn update_day_night_cycle(
    time: Res<Time>,
    mut day_night_cycle: ResMut<DayNightCycle>,
    mut query: Query<(&mut Sprite, &Position)>,
) {
    day_night_cycle.timer.tick(time.delta());

    // Calculate the current time of day as a percentage (0.0 - 1.0)
    let time_of_day = day_night_cycle.timer.elapsed_secs() / DAY_DURATION;

    // Calculate the sun brightness
    let global_brightness_factor = match time_of_day {
        t if t <= 0.1 => lerp(NIGHT_LIGHT_LEVEL, DAY_LIGHT_LEVEL, t / 0.1), // Dawn
        t if t <= 0.35 => DAY_LIGHT_LEVEL, // Day
        t if t <= 0.45 => lerp(DAY_LIGHT_LEVEL, NIGHT_LIGHT_LEVEL, (t - 0.35) / 0.1), // Dusk
        _ => NIGHT_LIGHT_LEVEL, // Night
    };

    let torch_multiplier = match time_of_day {
        t if t <= 0.1 => lerp(1., 0., t / 0.1), // Dawn
        t if t <= 0.35 => 0., // Day
        t if t <= 0.45 => lerp(0., 1., (t - 0.35) / 0.1), // Dusk
        _ => 1., // Night
    };

    // Calculate the brightness factor for this tile
    
    for (mut sprite, pos) in query.iter_mut() {
        // Deconstruct the position vector
        let Vec2 { x, y } = pos.0;

        let (x, y) = map::get_tile(x, y);

        let numbers = vec![
            map::distance_int_from_point((7, 6), (x, y)).floor() as i32,
            map::distance_int_from_point((7, 15), (x, y)).floor() as i32,
            map::distance_int_from_point((32, 6), (x, y)).floor() as i32,
            map::distance_int_from_point((32, 15), (x, y)).floor() as i32,
        ];

        let mut torch_light_factor = 1.;

        if let Some(min_value) = numbers.iter().cloned().min() {
            if min_value < 7 {
                let dist = 7 - min_value;
                torch_light_factor = 1. + (dist as f32 * (0.1) * torch_multiplier);
            }
        } else {
            println!("Vector is empty");
        }
        
        // Calculate total brightness factor (constain torch lights to 7 units away)
        let brightness_factor = global_brightness_factor * torch_light_factor;
        // your color changing logic here instead:
        sprite.color = Color::rgba(
            1.0 * brightness_factor,
            1.0 * brightness_factor,
            1.0 * brightness_factor,
            1.0, // Preserve the alpha channel
        );
    }
}

fn lerp (a: f32, b: f32, ratio: f32) -> f32 {
    a + ((b - a) * ratio)
} 
