use bevy::prelude::*;

use splash::SplashPlugin;
use state::{DisplayQuality, GameState, Volume};

mod game;
mod menu;
mod splash;
mod state;
mod util;

/// Set up the main app, add defaults and show the splash page.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
