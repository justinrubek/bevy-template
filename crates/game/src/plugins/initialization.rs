use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct InitializationPlugin;

impl Plugin for InitializationPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Initialization).continue_to_state(GameState::MainMenu),
        );
    }
}
