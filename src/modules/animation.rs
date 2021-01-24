
use sdl2::rect::Rect;
use specs::prelude::*;

use super::{Movement, PlayerStatus, Sprite};

pub struct Animation;

impl <'a> System<'a> for Animation {
    type SystemData  = (ReadStorage<'a, Movement>, WriteStorage<'a, Sprite>);

    fn run(&mut self, mut data: Self::SystemData) {
        for (mov, spr) in (&data.0, &mut data.1).join() {
            match mov.direction {
                PlayerStatus::WalkLeft => {
                   spr.source_rect = Rect::new(
                       mov.animation_frame * 16,
                       0,
                       16,
                       16
                   ); 
                },
                PlayerStatus::WalkRight => {
                    spr.source_rect = Rect::new(
                        mov.animation_frame * 16,
                        16,
                        16,
                        16
                    ); 
                },
                PlayerStatus::Stopped => {
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