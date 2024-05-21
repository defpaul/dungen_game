

use bevy::prelude::*;
use dungen_game::charecter::player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin{
                    primary_window: Some(Window {
                        title: "dungen".into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, player::movement)
        .run();
}

fn setup(
    mut commands: Commands, asset_server: Res<AssetServer>
){
    commands.spawn(Camera2dBundle::default());
    player::spawn(commands, asset_server)
}

