use sdl2::{rect::{Point, Rect}};
use specs::prelude::*;
use specs_derive::Component;

use super::PlayerStatus;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Player {
    pub speed: i32,
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
    pub direction: PlayerStatus
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
// pub struct Sprite<'a> {
// 	pub source_rect: Rect,
// 	pub sprite_sheet: &'a str,
// 	// bounding_box: Rectangle,
// 	// x: i32, 
//     // y: i32
// }

// impl Sprite<'_> {

//     pub fn new( filepath: &str, sourceX: i32, sourceY: i32, width: u32, height: u32,
//         posX: i32, posY: i32) -> Sprite {

        
//         Sprite{
//             source_rect: Rect::new(sourceX, sourceY, width, height) ,
//             sprite_sheet: filepath,
//         }
//     }


// }

