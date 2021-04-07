use specs::{Read, ReadStorage, System, WriteStorage, Join, ReadExpect};

use self::PlayerDirection::*;
use super::{PlayerDirection, Player, Movement};
use crate::modules::PlayerStatus;

const WALKING_SPEED: i32 = 10;
pub struct Input;

impl <'a> System<'a> for Input {
    type SystemData  =  (ReadExpect<'a, PlayerStatus>, ReadStorage<'a, Player>, WriteStorage<'a, Movement> );

    fn run(&mut self, mut data: Self::SystemData) {
        let (ps, pl, mut mov) = data;
        for (_, mov) in (&pl, &mut mov).join() {
            mov.is_jumping = ps.is_jumping;
            match ps.direction {
                Some(dir) => {
                    match dir {
                        Left => {
                            mov.direction = PlayerDirection::Left;
                            mov.speed = WALKING_SPEED;
                        }
                        Right => {
                            mov.direction = PlayerDirection::Right;
                            mov.speed = WALKING_SPEED;
                        }
                        Stopped => {
                            mov.direction = PlayerDirection::Stopped;
                            mov.speed = 0;
                            mov.animation_frame = 0;
                        }
                    }
                },
                None => {
                    // keep moving
                    if mov.animation_frame < 3 {
                        mov.animation_frame = mov.animation_frame + 1;
                    } else {
                        mov.animation_frame = 0;
                    }
                }
            }
        };
    }
}