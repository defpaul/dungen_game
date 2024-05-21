use bevy::prelude::*;

use crate::components;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
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
        components::Player{},
    ));
}

pub fn movement(
    mut query: Query<(&mut Transform, &components::Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
){
    for (mut transform, _player) in &mut query {
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
