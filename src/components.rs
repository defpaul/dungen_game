use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Player {}

#[derive(Component, Clone)]
pub struct Cam {}

pub const IMAGE_SCALE: Vec3 = Vec3::new(5., 5., 1.);

#[derive(Component,PartialEq,Clone)]
pub enum Puch {
    Yes,
    No,
    None,
}

#[derive(Component,PartialEq,Clone)]
pub struct Dir {
    pub direction: Direction,
}

#[derive(Component)]
pub struct Hit {
    pub just_hit:bool,
    pub item: Item,
}

#[derive(Component)]
pub struct Despawn{}

#[derive(Component)]
pub struct Level {}

#[derive(PartialEq, Clone)]
pub enum Direction {
   Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Component)]
pub  enum Item {
    Box,
    Enemy,
}