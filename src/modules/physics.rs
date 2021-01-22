
use specs::prelude::*;

use super::{PlayerStatus, Movement, Position};

pub struct Physics;

impl <'a> System<'a> for Physics {
    type SystemData  = (ReadStorage<'a, Movement>, WriteStorage<'a, Position>);

    fn run(&mut self, mut data: Self::SystemData) {
        for (mov, pos) in (&data.0, &mut data.1).join() {
            match mov.direction {
                PlayerStatus::WalkLeft => {
                    pos.point = pos.point.offset(-mov.speed, 0);
                },
                PlayerStatus::WalkRight => {
                    pos.point = pos.point.offset(mov.speed, 0);
                },
                PlayerStatus::Stopped => {

                }
            }
        }
    }
}