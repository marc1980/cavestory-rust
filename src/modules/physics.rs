use specs::prelude::*;

use super::{PlayerStatus, Movement, Position};

const GRAVITY: i32 = 3;

pub struct Physics;

impl <'a> System<'a> for Physics {
    type SystemData  = (ReadStorage<'a, Movement>, WriteStorage<'a, Position>);

    fn run(&mut self, mut data: Self::SystemData) {
        for (mov, pos) in (&data.0, &mut data.1).join() {
            match mov.direction {
                PlayerStatus::WalkLeft => {
                    pos.rect.offset(-mov.speed, GRAVITY);
                },
                PlayerStatus::WalkRight => {
                    pos.rect.offset(mov.speed, GRAVITY);
                },
                PlayerStatus::Stopped => {
                    pos.rect.offset(0, GRAVITY);
                }
            }
        }
    }
}