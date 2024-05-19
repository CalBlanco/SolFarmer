use bevy_kira_audio::prelude::*;
use bevy::{prelude::*, utils::tracing::instrument};
use std::time::Duration;

use crate::{game::*};


#[derive(Resource)]
pub struct DayMusicInstanceHandle(Handle<AudioInstance>);

#[derive(Resource)]
pub struct NightMusicInstanceHandle(Handle<AudioInstance>);

pub fn start_day_music(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    
    // Play the day music
    let handle = audio.play(asset_server.load("music/SolFarmer_4_Track_2.ogg"))
    // Fade-in with a dynamic easing
    .fade_in(AudioTween::new(Duration::from_secs(1), AudioEasing::OutPowi(2)))
    // Play at half volume
    .with_volume(0.5)
    // play the track reversed
    .looped().handle();
    // Add the resource
    commands.insert_resource(DayMusicInstanceHandle(handle));
}

pub fn start_night_music(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    // Play the day music
    let handle = audio.play(asset_server.load("music/SolFarmer_4_Track.ogg"))
    // Fade-in with a dynamic easing
    .fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::OutPowi(2)))
    // Play at half volume
    .with_volume(0.0)
    // play the track reversed
    .looped().handle();
    // Add the resource
    commands.insert_resource(NightMusicInstanceHandle(handle));
}

pub fn instance_control(
    day_handle: Res<DayMusicInstanceHandle>,
    night_handle: Res<NightMusicInstanceHandle>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut ev_dawn: EventReader<DawnStartEvent>,
    mut ev_dusk: EventReader<DuskStartEvent>,
) {

    // Handle Dawn
    for _ev in ev_dawn.read() {
        // "Lerp" day music volume up to 0.5
        if let Some(instance) = audio_instances.get_mut(&day_handle.0) {
            instance.set_volume(0.5, AudioTween::new(Duration::new(3, 0), AudioEasing::InPowf(2.)));
            // Reset Song
            instance.seek_to(0.0);
        }
        // "Lerp" night music volume down to 0.0
        if let Some(instance) = audio_instances.get_mut(&night_handle.0) {
            instance.set_volume(0.0, AudioTween::new(Duration::new(3, 0), AudioEasing::OutPowf(2.)));
        }
    }
    // Handle Dawn
    for _ev in ev_dusk.read() {
        // "Lerp" day music volume down to 0.0
        if let Some(instance) = audio_instances.get_mut(&day_handle.0) {
            instance.set_volume(0.0, AudioTween::new(Duration::new(3, 0), AudioEasing::OutPowf(2.)));
        }
        // "Lerp" night music volume up to 0.5
        if let Some(instance) = audio_instances.get_mut(&night_handle.0) {
            instance.set_volume(0.5, AudioTween::new(Duration::new(3, 0), AudioEasing::InPowf(2.)));
            // Reset Song
            instance.seek_to(0.0);
        }
    }
}