use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, prelude::*};


use super::{AppState, RESOLUTION_X, RESOLUTION_Y};

pub fn build_plugin(app: &mut App){
    app
    .add_systems(OnEnter(AppState::Game), setup)
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

