use std::fs;
use bevy::prelude::*;
use crate::components::{Level, LevelObjekt};

pub fn seleckt(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    let level:u8=1;
    loade(commands, asset_server, level)
}


pub fn reload(
    object: Query<Entity, With<LevelObjekt>>,
    mut commands: Commands,
    asset_sever: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>
) {
    if input.just_pressed(KeyCode::KeyR) {
        for entity in &object {
            commands.entity(entity).despawn_recursive()
        }
        loade(commands, asset_sever, 0)
    }
}

pub fn loade(
   mut command: Commands,
   asset_server: Res<AssetServer>,
   level:u8
){


    let mut posx: f32 = 0.0;
    let mut posy: f32 = 0.0;
    let mut is_player:bool = false;

    let mut level_lode:String="".to_string();

    match level {
        1 => {level_lode = fs::read_to_string("level/level1.txt").unwrap()}
        _=>()
    }


    for i in level_lode.lines() {
        for j in i.chars() {
            match j {
                '#' => spawn::wall(posx, posy, &mut command, &asset_server),
                '_' => spawn::flor(posx, posy, &mut command, &asset_server),
                '?' => spawn::r#box(posx, posy, &mut command, &asset_server),
                '°' => {
                    if !is_player {
                        spawn::player(posx, posy, &mut command, &asset_server, );
                        is_player=true
                    }else {spawn::flor(posx, posy, &mut command, &asset_server)}
                },
                _ => (),
            }
            posx += 50.0;
        }
        posy -= 50.0;
        posx = 0.0;
    }
    if !is_player {panic!()}
}

mod spawn{
    use bevy::prelude::*;
    use crate::components;
    use crate::charecter::player;
    use crate::components::LevelObjekt;

    pub fn flor
    (
        posx: f32,
        posy: f32,
        command: &mut Commands,
        asset_server: &Res<AssetServer>
    ){
        command.spawn((
            SpriteBundle{
                texture: asset_server.load("flor.png"),
                transform: Transform {
                    translation: Vec3::new(posx, posy,0.0),
                    scale: components::IMAGE_SCALE,
                    ..default()
                },
                ..default()
            },
            components::Puch::None,
            LevelObjekt{}
        ));
    }

    pub fn wall(
        posx: f32,
        posy: f32,
        command: &mut Commands,
        asset_server: &Res<AssetServer>
    ){
        command.spawn((
            SpriteBundle{
                texture: asset_server.load("wall.png"),
                transform: Transform {
                    translation: Vec3::new(posx, posy, 0.0),
                    scale: components::IMAGE_SCALE,
                    ..default()
                },
                ..default()
            },
            components::Puch::No,
            LevelObjekt{}
        ));
    }

    pub fn r#box(
        posx: f32,
        posy: f32,
        command: &mut Commands,
        asset_server: &Res<AssetServer>
    ){

        command.spawn((
            SpriteBundle{
                texture: asset_server.load("box.png"),
                transform: Transform {
                    translation: Vec3::new(posx, posy, 0.),
                    scale: components::IMAGE_SCALE,
                    ..default()
                },
                ..default()
            },
            components::Puch::No,
            components::Hit{
                just_hit: false,
                item: components::Item::Box,
            },
            LevelObjekt{}
        ));

       // flor(posx, posy, command, asset_server);
}

    pub fn player(
        posx: f32,
        posy: f32,
        command: &mut Commands,
        asset_server: &Res<AssetServer>,
    ){
        player::spawn(posx, posy, command, asset_server);
        flor(posx, posy, command, &asset_server);
    }
}