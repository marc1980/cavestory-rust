
extern crate sdl2;

use std::{collections::HashMap};
use sdl2::{Sdl, image::{InitFlag}, init, rect::Rect, render::{Canvas, Texture}, video::{Window}};

use components::Sprite;

use super::{SpriteSheet, components, level::TileMap};
use crate::modules::Rectangle;

pub struct Graphics {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
}

impl Graphics {
    pub fn new() -> Graphics {
        let sdl_context = init().expect("failed to init SDL");
        let _image_context = sdl2::image::init(InitFlag::PNG).expect("failed to initialize image context");
        let video_subsystem = sdl_context.video().expect("failed to get video context");
        let window = video_subsystem
            .window("Cavestory", 640, 480)
            .position_centered()
            .opengl()
            .build()
            .expect("failed to build window");
        let canvas = window.into_canvas().build().map_err(|e| e.to_string()).expect("failed to create canvas");

        Graphics{
            sdl_context, 
            canvas,
        }
    }
    
    pub fn render(&mut self, sprite: &Sprite, location: Rectangle, background: &TileMap, textures: &HashMap<SpriteSheet, Texture>) {

       self.canvas.clear();

        // render level
        let texture = textures.get(&background.tiles_texture).unwrap();
        for (idx, src) in background.source.iter().enumerate() {
            let dst = &background.dest[idx];
            self.canvas.copy(texture, src.clone(), dst.clone()).expect("failed to render tile");
        }

        // render player
        let player_texture = textures.get(&sprite.spritesheet).unwrap();
        let destination_rect = Rect::new(
            location.x.round() as i32,
            location.y.round() as i32,
            location.width.round() as u32,
            location.height.round() as u32);

        self.canvas.copy( player_texture, sprite.source_rect, destination_rect).expect("texture copy failed");
        self.canvas.present();
    }
}

