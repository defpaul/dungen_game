use bevy::prelude::*;

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
        .add_systems(Update, move_player)
        .run();
 
}

#[derive(Component)]
struct Player {}

fn setup(
    mut commands: Commands, asset_server: Res<AssetServer>
){
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("priest1_v1_1.png"),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(5., 5., 1.),
                ..default()
            },
            ..default() 
        },
        Player{},
    ));
}

fn move_player(
    mut query: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
){
    for (mut transform, player) in &mut query {
       if input.pressed(KeyCode::KeyW) {
            transform.translation.y += 100. * time.delta_seconds()  
       } 
       if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 100. * time.delta_seconds()
       }
       if input.pressed(KeyCode::KeyD) {
            transform.translation.x += 100. * time.delta_seconds() 
       }
       if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 100. * time.delta_seconds() 
       }
    }
}
