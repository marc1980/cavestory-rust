use specs::prelude::*;

use super::{CollisionTargets, Position};
use crate::modules::CollisionType;

// const GRAVITY: i32 = 3;

pub struct Collision;

impl <'a> System<'a> for Collision {
    type SystemData  = (ReadStorage<'a, CollisionTargets>, WriteStorage<'a, Position>);

    fn run(&mut self, mut data: Self::SystemData) {
        for (ct, mut pos) in (&data.0, &mut data.1).join() {
           // println!("pos: {}, {} - {}, {}", pos.rect.x, pos.rect.y, pos.rect.getRight(), pos.rect.getBottom());
            for col in ct.collisions.blocks.iter() {
                if pos.rect.collides_with(col) {
                    let side = pos.rect.get_colliding_side(col);
                   // println!("col: {}, {} - {}, {} -- {:?}", col.x, col.y, col.getRight(), col.getBottom(), side);
                    match side {
                        Some(CollisionType::Bottom) => {
                            pos.rect.y = col.y - pos.rect.height;
                            pos.is_grounded = true;
                        },
                        Some(CollisionType::Top) => pos.rect.y = col.y + col.height,
                        Some(CollisionType::Left) => pos.rect.x = col.x + col.width,
                        Some(CollisionType::Right) => pos.rect.x = col.x - pos.rect.width,
                        _ => return
                    }
                }
            }
        }
    }
}