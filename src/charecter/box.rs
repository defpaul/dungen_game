use bevy::prelude::*;
use crate::components::{Despawn, Hit, Item, Puch};

pub fn hit_detektion(
    asset_server: Res<AssetServer>,
    mut objekts: Query<(Entity, &mut Puch ,&mut Hit, &mut Handle<Image>), With<Hit>>
){
    for (_entity, mut puch, mut hit, mut image) in &mut objekts {
        if hit.just_hit {
            match hit.item {
                Item::Box => {*puch= Puch::None;*image=asset_server.load("flor.png");} ,
                _ => ()
            }
        };
        hit.just_hit = false
    }
}


pub fn despawn(
    mut commands: Commands,
    objekts: Query<Entity, With<Despawn>>
){
    for objekt in objekts.iter() {
        commands.entity(objekt).despawn()

    }
}