use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*, window::{PresentMode, WindowTheme}};
use bevy_hanabi::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin, AudioSource};

use bevy::winit::WinitWindows;
use winit::window::Icon;

const RESOLUTION_X: f32 = 1312.;
const RESOLUTION_Y: f32 = 704.;

mod menu;
mod game;
mod map;
mod player;
mod the_core;
mod music_player;

mod spriteanims;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "SolFarmer".into(),
                    name: Some("SolFarmer".into()),
                    resolution: (RESOLUTION_X, RESOLUTION_Y).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    resize_constraints: WindowResizeConstraints { min_width: RESOLUTION_X, min_height: RESOLUTION_Y, max_width: RESOLUTION_X, max_height: RESOLUTION_Y },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    
                    ..default()
                }),
                ..default()
            }),
            //LogDiagnosticsPlugin::default(),
            //FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(HanabiPlugin) // Particle Effect System
        .add_plugins(AudioPlugin)
        .add_plugins(menu::build_plugin)
        .add_plugins(game::build_plugin)
        .add_systems(Startup, set_window_icon) // Set the application icon
        .init_state::<AppState>()
        .run();
}




fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/images/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

