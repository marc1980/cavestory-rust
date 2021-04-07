
use sdl2::rect::Rect;
use specs::prelude::*;

use super::{Movement, PlayerDirection, Sprite};

pub struct Animation;

impl <'a> System<'a> for Animation {
    type SystemData  = (ReadStorage<'a, Movement>, WriteStorage<'a, Sprite>);

    fn run(&mut self, mut data: Self::SystemData) {
        for (mov, spr) in (&data.0, &mut data.1).join() {
            match mov.direction {
                PlayerDirection::Left => {
                   spr.source_rect = Rect::new(
                       mov.animation_frame * 16,
                       0,
                       16,
                       16
                   ); 
                }
                PlayerDirection::Right => {
                    spr.source_rect = Rect::new(
                        mov.animation_frame * 16,
                        16,
                        16,
                        16
                    ); 
                }
                PlayerDirection::Stopped => {
                    spr.source_rect = Rect::new(
                        0,
                        0,
                        16,
                        16
                    ); 
                }
            }
        }
    }
}