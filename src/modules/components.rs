use sdl2::rect::Rect;
use specs::prelude::*;
use specs_derive::Component;

use super::PlayerDirection;
use crate::modules::level::Collisions;
use std::cmp::Ordering::Equal;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Player {
    pub animation_frame: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub rect: Rectangle,
    pub is_grounded: bool
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Movement {
    pub speed: i32,
    pub direction: PlayerDirection,
    pub animation_frame: i32,
    pub is_jumping: bool
}

#[derive(Component, Debug, Copy, Clone)]
#[storage(VecStorage)]
pub struct Slope {
    pub p1: Point,
    pub p2: Point
}

impl Slope {
    pub fn get_slope(&self) -> f32 {
        if self.p2.x - self.p1.x != 0f32 {
            return (self.p2.y.abs() - self.p1.y.abs()) / (self.p2.x.abs() - self.p1.x.abs());
        }
        return 0f32;
    }
}

#[derive(Component, Debug, Copy, Clone)]
#[storage(VecStorage)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

impl Rectangle {
    pub fn get_left(&self) -> f32 { return self.x; }
    pub fn get_right(&self) -> f32 { return self.x + self.width; }
    pub fn get_top(&self) -> f32 { return self.y; }
    pub fn get_bottom(&self) -> f32 { return self.y + self.height; }
    pub fn offset(&mut self, x: i32, y: i32) {
        self.x += x as f32;
        self.y += y as f32;
    }
    pub fn offset_y(&mut self, y: i32) { self.y += y as f32; }
    pub fn collides_with_rectangle(&self, other: &Rectangle) -> bool {
        return self.get_right() >= other.get_left() &&
            self.get_left() <= other.get_right() &&
            self.get_top() <= other.get_bottom() &&
            self.get_bottom() >= other.get_top();
    }
    pub fn collides_with_slope(&self, other: &Slope) -> bool {
        return
            (self.get_right() >= other.p2.x &&
                self.get_left() <= other.p1.x &&
                self.get_top() <= other.p2.y &&
                self.get_bottom() >= other.p1.y) ||
                (self.get_right() >= other.p1.x &&
                    self.get_left() <= other.p2.x &&
                    self.get_top() <= other.p1.y &&
                    self.get_bottom() >= other.p2.y) ||
                (self.get_left() <= other.p1.x &&
                    self.get_right() >= other.p2.x &&
                    self.get_top() <= other.p1.y &&
                    self.get_bottom() >= other.p2.y) ||
                (self.get_left() <= other.p2.x &&
                    self.get_right() >= other.p1.x &&
                    self.get_top() <= other.p2.y &&
                    self.get_bottom() >= other.p1.y);
    }
    pub fn get_colliding_side(&self, other: &Rectangle) -> Option<CollisionType> {
        let amount_left = other.get_right() - self.get_left();
        let amount_right = self.get_right() - other.get_left();
        let amount_top = other.get_bottom() - self.get_top();
        let amount_bottom = self.get_bottom() - other.get_top();
        let mut lowest = vec![amount_right.abs(), amount_left.abs(), amount_bottom.abs(), amount_top.abs()];
        lowest.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
        match lowest.first() {
            Some(min) if *min == amount_left => Some(CollisionType::Left),
            Some(min) if *min == amount_right => Some(CollisionType::Right),
            Some(min) if *min == amount_top => Some(CollisionType::Top),
            Some(min) if *min == amount_bottom => Some(CollisionType::Bottom),
            _ => None
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

#[derive(Debug)]
pub enum CollisionType {
    Top,
    Bottom,
    Left,
    Right
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct CollisionTargets {
    pub collisions: Collisions,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: SpriteSheet,
    pub source_rect: Rect,
}
#[derive(Debug, serde::Deserialize, PartialEq, Eq, Hash, Copy, Clone)]
pub enum SpriteSheet {
    MyChar,
    PrtCave,
    NpcSym,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Render {
    pub sprite: Sprite,
    pub destination_rect: Rect,
}
