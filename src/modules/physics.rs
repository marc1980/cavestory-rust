use specs::prelude::*;

use super::{PlayerDirection, Movement, Position};

const GRAVITY: i32 = 10;
const JUMP: i32 = -55;

pub struct Physics;

impl <'a> System<'a> for Physics {
    type SystemData  = (ReadStorage<'a, Movement>, WriteStorage<'a, Position>);

    fn run(&mut self, mut data: Self::SystemData) {
        for (mov, pos) in (&data.0, &mut data.1).join() {
            if mov.is_jumping && pos.is_grounded {
                pos.rect.offset_y(JUMP);
                pos.is_grounded = false;
            }
            match mov.direction {
                PlayerDirection::Left => {
                    pos.rect.offset(-mov.speed, GRAVITY);
                }
                PlayerDirection::Right => {
                    pos.rect.offset(mov.speed, GRAVITY);
                }
                PlayerDirection::Stopped => {
                    pos.rect.offset(0, GRAVITY);
                }
            }
        }
    }
}