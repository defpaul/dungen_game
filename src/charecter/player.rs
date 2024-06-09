use bevy::prelude::*;

use crate::components;
use crate::components::Direction::{Down, Right, Up, Left};
use crate::components::{Hit, Level};

#[derive(Component)]
pub struct PlayerPlugin {}
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, new_movement)
            .add_systems(Update, animation)
            .add_systems(Update, hiting);
    }
}
pub fn spawn(
    posx: f32,
    posy: f32,
    command: &mut Commands,
    asset_server: &Res<AssetServer>,
){
        command.spawn((
        SpriteBundle {
            texture: asset_server.load("player/player.png"),
            transform: Transform {
                translation: Vec3::new(posx, posy, 0.1),
                scale: components::IMAGE_SCALE,
                ..default()
            },
            ..default()
        },
//        components::Puch::No,
        components::Player{},
        components::Dir{direction: Up},
        Level{}
    ));
}


fn  new_movement(
    mut transform: ParamSet<(
        Query<(&mut Transform, &mut components::Dir, &components::Player)>,
        Query<(&Transform, &components::Puch)>,
    )>,
   input: Res<ButtonInput<KeyCode>>,
){

    let objekts: Vec<(Transform, components::Puch)> = transform.p1().iter().map(|(t,p)| (t.clone(), p.clone())).collect();
    let mut previous_pos:Vec3 = Vec3::new(0.0, 0.0, 0.0);

    for (mut transform_p, mut dir, _player) in  transform.p0().iter_mut() {
        let mut posx: f32 = transform_p.translation.x;
        let mut posy: f32 = transform_p.translation.y;

        if input.just_pressed(KeyCode::KeyW) {posy += 50.0; *dir=components::Dir{direction: Up}}
        if input.just_pressed(KeyCode::KeyS) {posy -= 50.0; *dir=components::Dir{direction: Down}}
        if input.just_pressed(KeyCode::KeyD) {posx += 50.0; transform_p.scale.x=5.; *dir=components::Dir{direction: Right}}
        if input.just_pressed(KeyCode::KeyA) {posx -= 50.0; transform_p.scale.x=-5.; *dir=components::Dir{direction: Right}}

        for (transform_o, objekts) in &objekts {
            if transform_o.translation.x == posx && transform_o.translation.y == posy {
                if transform_o.translation == previous_pos {continue}
                if objekts == &components::Puch::No {
                    previous_pos = transform_o.translation;
                    continue;
                }
                else if objekts == &components::Puch::None {

                    transform_p.translation.x = posx;
                    transform_p.translation.y = posy;
                }
                else if objekts == &components::Puch::Yes {
                    println!("Yes");
                    transform_p.translation.x = posx;
                    transform_p.translation.y = posy;
                }
            }
        }
    }
}

fn animation(
    asset_server:Res<AssetServer>,
    mut player:Query<(&mut Handle<Image>, &components::Dir)>,
){
    for (mut texture, dir) in &mut player {
        let up=asset_server.load("player/player_up.png");
        let down=asset_server.load("player/player_down.png");
        let std=asset_server.load("player/player.png");

        match dir.direction {
            Right => {*texture=std;},
            Left => {*texture=std;},
            Up => *texture=up,
            Down => *texture=down,
            _=>(),
        };
    }
}

fn hiting(
    input: Res<ButtonInput<KeyCode>>,
    mut objekts: ParamSet<(
        Query<(&Transform, &components::Player)>,
        Query<(&Transform, &mut Hit)>,
    )>,
){
    let player: Vec<(Transform, components::Player)> = objekts.p0().iter().map(|(t,p)| (t.clone(), p.clone())).collect();

    for (transform_p, _player) in player.iter() {

        let posx_p = transform_p.translation.x;
        let posy_p = transform_p.translation.y;

        for (transform_o, mut hit) in objekts.p1().iter_mut() {

            let posx_o = transform_o.translation.x;
            let posy_o = transform_o.translation.y;

            if input.just_pressed(KeyCode::KeyE) {
                if posx_p+50. == posx_o && posy_p == posy_o {hit.just_hit=true}
                if posx_p-50. == posx_o && posy_p == posy_o {hit.just_hit=true}
                if posx_p == posx_o && posy_p+50. == posy_o {hit.just_hit=true}
                if posx_p == posx_o && posy_p-50. == posy_o {hit.just_hit=true}
            }
        }
    }
}