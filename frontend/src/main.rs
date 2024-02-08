use bevy::prelude::*;

mod components;
mod entity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // .add_systems(Update, player_movement)
        .run();
}

#[derive(Component)]
struct MyCameraMarker;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        MyCameraMarker,
    ));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("buttle_of_water.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        ..Default::default()
    },));
}
