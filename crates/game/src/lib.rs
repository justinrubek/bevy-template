use crate::plugins::{initialization::InitializationPlugin, menu::MenuPlugin};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use plugins::input::InputPlugin;
use plugins::player::PlayerPlugin;

mod plugins;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Initialization,
    MainMenu,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            InitializationPlugin,
            MenuPlugin,
            InputPlugin,
            PlayerPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
