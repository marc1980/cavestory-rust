extern crate sdl2;
use sdl2::{EventPump, event::Event, image::LoadTexture, rect::Point};
use sdl2::keyboard::Keycode;
use std::{cmp, collections::HashMap, time::Instant};

use super::{Graphics, Sprite};

const MAX_FPS: u128 = 50;
const MAX_FRAME_TIME: u128 = 1000 / MAX_FPS; 

pub struct Game {
    pub graphics: Graphics,
    LAST_UPDATE_TIME: u128
}

impl Game {
    pub fn new() -> Game {
        return Game {
            graphics: Graphics::new(),
            LAST_UPDATE_TIME: Instant::now().elapsed().as_millis()
        }
    }

    pub fn game_loop(&mut self) {

        let texture_creator = self.graphics.canvas.texture_creator();
        let mut spritesheets =  HashMap::new();
        spritesheets.insert("./content/sprites/MyChar.png".to_string(),
                        texture_creator.load_texture("./content/sprites/MyChar.png").unwrap());

        

        let mut event_pump: EventPump = self.graphics.sdl_context.event_pump().expect("failed to load event pump");

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
    
            let now = Instant::now().elapsed().as_millis();
            let elapsed= cmp::min(now, MAX_FRAME_TIME);
            self.LAST_UPDATE_TIME = now;

            update(elapsed);
    
            self.graphics.clear();

            self.graphics.render(Sprite::new( 
                "./content/sprites/MyChar.png", 
                0, 0, 
                16, 16, 
                100, 100),
                Point::new(100, 100), spritesheets.get("./content/sprites/MyChar.png").unwrap());
            
            self.graphics.flip();

        }
    }
}

fn update(elapsed: u128) {
    println!("update: {}", elapsed);
}




