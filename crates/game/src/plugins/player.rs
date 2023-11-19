use crate::{
    plugins::{initialization::ImageAssets, input::UserInput},
    GameState,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(mut commands: Commands, images: Res<ImageAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: images.player.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        })
        .insert(Player);

    println!("Spawned player");
}

fn player_movement(
    time: Res<Time>,
    input: Res<UserInput>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    if input.player_movement.is_none() {
        return;
    }

    let speed = 100.0;
    let movement = Vec3::new(
        input.player_movement.unwrap().x * speed * time.delta_seconds(),
        input.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.0,
    );

    for mut transform in &mut player {
        transform.translation += movement;
    }
}
