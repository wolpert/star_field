use bevy::prelude::*;
use bevy::window::WindowTheme;

use splash::SplashPlugin;
use state::{DisplayQuality, GameState, Volume};

mod game;
mod menu;
mod splash;
mod state;
mod util;
mod config;

/// Set up the main app, add defaults and show the splash page.
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
                    resolution: (768., 1024.).into(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }))
        .init_resource::<config::StarFieldConfig>()
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume::new(7))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        // Adds the plugins for each state
        .add_plugins((SplashPlugin, menu::MenuPlugin, game::GamePlugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
