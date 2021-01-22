extern crate sdl2;
use sdl2::{EventPump, event::Event, image::LoadTexture, rect::{Point, Rect}};
use sdl2::keyboard::Keycode;
use specs::prelude::*;
use std::{cmp, collections::HashMap, time::Instant};
use components::{Sprite, SpriteSheet};
use super::{PlayerStatus, Graphics, Input, Player, Movement, Physics, Position, components};

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
        spritesheets.insert(SpriteSheet::MyChar, texture_creator.load_texture("./content/sprites/MyChar.png").unwrap());

        let mut world = World::new();
        world.register::<Player>();
        world.register::<Position>();
        world.register::<Movement>();
    
        let player = world.create_entity()
            .with(Player {speed: 0})
            .with(Position{point: Point::new(100, 100)})
            .with(Movement {speed: 0, direction: PlayerStatus::WalkRight})
            .build();

        let mut dispatcher = DispatcherBuilder::new()
            .with(Input, "input", &[])
            .with(Physics, "physics", &["input"])
            .build();

        dispatcher.setup(&mut world);       
        
        let mut event_pump: EventPump = self.graphics.sdl_context.event_pump().expect("failed to load event pump");
        
        'running: loop {
            let mut player_status_event: Option<PlayerStatus> = None;
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                        player_status_event = Some(PlayerStatus::WalkLeft);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                        player_status_event = Some(PlayerStatus::WalkRight);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                    Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. }  => {
                        player_status_event = Some(PlayerStatus::Stopped);
                    },
                        _ => {}
                    }
            }
    
            let now = Instant::now().elapsed().as_millis();
            let elapsed= cmp::min(now, MAX_FRAME_TIME);
            self.LAST_UPDATE_TIME = now;

            world.insert(player_status_event);

            dispatcher.dispatch(&mut world);


            update(elapsed);
    
            self.graphics.clear();

            let player_pos = world.read_component::<Position>();
            let player_point = player_pos.get(player).unwrap();
            self.graphics.render(Sprite { spritesheet: SpriteSheet::MyChar, source_rect: Rect::new(0, 0, 16, 16)},
            player_point.point, spritesheets.get(&SpriteSheet::MyChar).unwrap());
            
            self.graphics.flip();

        }
    }
}

fn update(elapsed: u128) {
   // println!("update: {}", elapsed);
}




