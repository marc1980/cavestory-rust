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
            for rect in ct.collisions.blocks.iter() {
                if pos.rect.collides_with_rectangle(rect) {
                    let side = pos.rect.get_colliding_side(rect);
                   // println!("col: {}, {} - {}, {} -- {:?}", col.x, col.y, col.getRight(), col.getBottom(), side);
                    match side {
                        Some(CollisionType::Bottom) => {
                            pos.rect.y = rect.y - pos.rect.height;
                            pos.is_grounded = true;
                        },
                        Some(CollisionType::Top) => pos.rect.y = rect.y + rect.height,
                        Some(CollisionType::Left) => pos.rect.x = rect.x + rect.width,
                        Some(CollisionType::Right) => pos.rect.x = rect.x - pos.rect.width,
                        _ => return
                    }
                }
            }
            for slope in ct.collisions.slopes.iter() {
                if pos.rect.collides_with_slope(slope) {
                    //Calculate where on the slope the player's bottom center is touching
                    //and use y=mx+b to figure out the y position to place him at
                    //First calculate "b" (slope intercept) using one of the points (b = y - mx)
                    let b: f32 = (slope.p1.y - (slope.get_slope() * slope.p1.x.abs()));

                    //Now get player's center x
                    let centerX: f32 = pos.rect.x + (pos.rect.width / 2f32);

                    //Now pass that X into the equation y = mx + b (using our newly found b and x)
                    // to get the new y position
                    let newY = (slope.get_slope() * centerX) + b;

                    //Re-position the player to the correct "y"
                    if pos.is_grounded {
                        pos.rect.y = newY - pos.rect.height;
                        pos.is_grounded = true;
                    }
                }
            }
        }
    }
}