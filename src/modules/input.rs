use specs::{Read, ReadStorage, System, WriteStorage, Join};

use self::PlayerStatus::*;
use super::{PlayerStatus, Player, Movement};

pub struct Input;

impl <'a> System<'a> for Input {
    type SystemData  =  (Read<'a, Option<PlayerStatus>>, ReadStorage<'a, Player>, WriteStorage<'a, Movement> );

    fn run(&mut self, mut data: Self::SystemData) {
        match *data.0 {
            Some(dir) => {
                for (_, mov) in (&data.1, &mut data.2).join() {
                    match dir {
                        WalkLeft => {
                            mov.direction = PlayerStatus::WalkLeft;
                            mov.speed = 1;
                        },
                        WalkRight => {
                            mov.direction = PlayerStatus::WalkRight;
                            mov.speed = 1;
                        },
                        Stopped => {
                            mov.speed = 0;
                        }
                    }
                }
            },
            None => {
                // keep moving
            }
        };

    }
}