use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, prelude::*};

use super::AppState;

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
            ..default()
        },
        BloomSettings{
            intensity: 0.1,
            ..default()
        },
    ));


    commands.spawn(
        SpriteBundle {
            texture: assets.load("tiles/redgrass.png"),
            transform: Transform::from_xyz(0.,0., 0.),
            ..default()
        }
    );
}


fn cleanup(mut commands: Commands, query: Query<(Entity, &Transform)>,  cams: Query<Entity, With<Camera>>,){
    for (e, _) in query.iter(){
        commands.entity(e).despawn();
    }

    for ent in &cams{
        commands.entity(ent).despawn();
    }
}

