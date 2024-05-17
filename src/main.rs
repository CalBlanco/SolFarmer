use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*, window::{PresentMode, WindowTheme}};
use bevy_hanabi::prelude::*;

const RESOLUTION_X: f32 = 1280.;
const RESOLUTION_Y: f32 = 720.;


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
                        maximize: true,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(HanabiPlugin)
        .run();
}