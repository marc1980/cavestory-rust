use sdl2::{rect::{Point, Rect}};
use specs::prelude::*;
use specs_derive::Component;

use super::PlayerStatus;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Player {
    pub animation_frame: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub point: Point,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Movement {
    pub speed: i32,
    pub direction: PlayerStatus,
    pub animation_frame: i32
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: SpriteSheet,
    pub source_rect: Rect,
}
#[derive(Debug, serde::Deserialize, PartialEq, Eq, Hash, Copy, Clone)]
pub enum SpriteSheet {
    MyChar,
    PrtCave,
    NpcSym,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Render {
    pub sprite: Sprite,
    pub destination_rect: Rect,
}
