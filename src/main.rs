use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use dungen_game::map::level;
use dungen_game::components;
use dungen_game::charecter::player::PlayerPlugin;
use dungen_game::charecter::r#box::{despawn, hit_detektion};
use dungen_game::map::level::reload_level;

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
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PlayerPlugin{})

        .add_systems(Startup, setup)
        .add_systems(Startup, level::lode_level)
//        .add_systems(Update, player::movement)
        .add_systems(Update, move_cam)
        .add_systems(Update, despawn)
        .add_systems(Update, hit_detektion)
        .add_systems(Update, reload_level)
        .run();
}

fn setup(
    mut commands: Commands,
//    asset_server: Res<AssetServer>
){
    commands.spawn((Camera2dBundle::default(),components::Cam{}));
    //player::spawn(commands, asset_server);
}

fn move_cam(
    mut objekts: ParamSet<(
        Query<(&mut Transform, &components::Cam)>,
        Query<(&Transform, &components::Player)>,
    )>,
    time: Res<Time>
){
    let player:Vec<(Transform, components::Player)> = objekts.p1().iter().map(|(t, p)| (t.clone(), p.clone())).collect();
    let ofset:f32 = 50.*2.;
    let speed:f32 = 150.;

    for (mut transform_c, _cam) in objekts.p0().iter_mut() {
        for (transform_p, _player) in &player {
            if transform_c.translation.x >= transform_p.translation.x+ofset {
                transform_c.translation.x-=speed*time.delta_seconds();
            }
            if transform_c.translation.x <= transform_p.translation.x-ofset {
                transform_c.translation.x+=speed*time.delta_seconds();
            }
            if transform_c.translation.y >= transform_p.translation.y+ofset {
                transform_c.translation.y-=speed*time.delta_seconds();
            }
            if transform_c.translation.y <= transform_p.translation.y-ofset {
                transform_c.translation.y+=speed*time.delta_seconds();
            }
        }
    }
}