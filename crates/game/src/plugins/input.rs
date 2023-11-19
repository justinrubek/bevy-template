use crate::GameState;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UserInput>().add_systems(
            Update,
            calculate_movement.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct UserInput {
    pub player_movement: Option<Vec2>,
}

pub fn calculate_movement(mut input: ResMut<UserInput>, keyboard_input: Res<Input<KeyCode>>) {
    let player_movement = Vec2::new(
        determine_movement(GameInput::MoveRight, &keyboard_input)
            - determine_movement(GameInput::MoveLeft, &keyboard_input),
        determine_movement(GameInput::MoveUp, &keyboard_input)
            - determine_movement(GameInput::MoveDown, &keyboard_input),
    );

    if player_movement != Vec2::ZERO {
        input.player_movement = Some(player_movement.normalize());
    } else {
        input.player_movement = None;
    }
}

pub enum GameInput {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    #[allow(dead_code)]
    Interact,
}

impl GameInput {
    pub fn pressed(&self, input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameInput::MoveUp => input.pressed(KeyCode::W),
            GameInput::MoveDown => input.pressed(KeyCode::S),
            GameInput::MoveLeft => input.pressed(KeyCode::A),
            GameInput::MoveRight => input.pressed(KeyCode::D),
            GameInput::Interact => input.pressed(KeyCode::E),
        }
    }
}

pub fn determine_movement(player_input: GameInput, input: &Res<Input<KeyCode>>) -> f32 {
    if player_input.pressed(input) {
        1.0
    } else {
        0.0
    }
}
