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

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    /// The specific spritesheet to render from
    pub spritesheet: SpriteSheet,
    /// The current region of the spritesheet to be rendered
    pub source_rect: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteSheet {
    MyChar,
}


