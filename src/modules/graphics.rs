
extern crate sdl2;

use std::{collections::HashMap};
use sdl2::{Sdl, image::{InitFlag, LoadTexture}, init, rect::{Point, Rect}, render::{Canvas, Texture, TextureCreator}, video::{Window, WindowContext}};

use components::Sprite;

use super::components;

pub struct Graphics {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
   // texture_creator: TextureCreator<WindowContext>,
   // spritesheets: HashMap<String, Texture>,
}

impl Graphics {
    pub fn new() -> Graphics {
        let sdl_context = init().expect("failed to init SDL");
        let _image_context = sdl2::image::init(InitFlag::PNG).expect("failed to initialize image context");
        let video_subsystem = sdl_context.video().expect("failed to get video context");
        let window = video_subsystem
            .window("Cavestory", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .expect("failed to build window");
        let canvas = window.into_canvas().build().map_err(|e| e.to_string()).expect("failed to create canvas");
        // let texture_creator = canvas.texture_creator();
        // let mut spritesheets =  HashMap::new();
        // spritesheets.insert("./content/sprites/MyChar.png".to_string(),
        //                 texture_creator.load_texture("./content/sprites/MyChar.png").unwrap());
        Graphics{
            sdl_context, 
            canvas,
          //  texture_creator,
           // spritesheets,
        }
    }
    
    pub fn render(&mut self, sprite: &Sprite, location: Point, texture: &Texture) {

       self.canvas.clear();

        let destination_rect = Rect::new(
            location.x, 
            location.y,
            sprite.source_rect.width() *2,
            sprite.source_rect.height() *2);

        self.canvas.copy( texture, sprite.source_rect, destination_rect).expect("texture copy failed");
        self.canvas.present();
    }
}

