extern crate sdl2;
use sdl2::{EventPump, event::Event, image::LoadTexture, rect::{Point, Rect}};
use sdl2::keyboard::Keycode;
use specs::prelude::*;
use std::{collections::HashMap, thread, time::{Duration, Instant}};
use components::{Sprite, SpriteSheet};
use super::{Animation, Graphics, Input, Level, Movement, Physics, Player, PlayerStatus, Position, components};

const MAX_FPS: u64 = 20;
const MAX_FRAME_TIME: u64 = 1000 / MAX_FPS; 

pub struct Game {
    pub graphics: Graphics,
    last_update_time: Instant,
    level: Level
}

impl Game {
    pub fn new() -> Game {
        return Game {
            graphics: Graphics::new(),
            last_update_time: Instant::now(),
            level: Level::new()
        }
    }

    pub fn game_loop(&mut self) {

        let texture_creator = self.graphics.canvas.texture_creator();
        let mut spritesheets =  HashMap::new();
        spritesheets.insert(SpriteSheet::MyChar, texture_creator.load_texture("./content/sprites/MyChar.png").unwrap());
        spritesheets.insert(SpriteSheet::PrtCave, texture_creator.load_texture("./content/sprites/PrtCave.png").unwrap());

        let mut world = World::new();
        world.register::<Player>();
        world.register::<Position>();
        world.register::<Movement>();
        world.register::<Sprite>();
    
        let player = world.create_entity()
            .with(Player {animation_frame: 0})
            .with(Position {point: Point::new(100, 100)})
            .with(Movement {speed: 0, direction: PlayerStatus::Stopped, animation_frame: 0})
            .with(Sprite { spritesheet: SpriteSheet::MyChar, source_rect: Rect::new(0, 0, 16, 16)})
            .build();

        let mut dispatcher = DispatcherBuilder::new()
            .with(Input, "input", &[])
            .with(Physics, "physics", &["input"])
            .with(Animation, "animation", &["input"])
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

            world.insert(player_status_event);

            dispatcher.dispatch(&mut world);
            world.maintain();
    
            let player_pos = world.read_component::<Position>().get(player).unwrap().point;
            self.graphics.render(world.read_component::<Sprite>().get(player).unwrap(),
                player_pos, &self.level.get_tile_map(), &spritesheets);
            
            let elapsed = self.last_update_time.elapsed().as_millis() as u64;

            if elapsed < MAX_FRAME_TIME {
                thread::sleep(Duration::from_millis(MAX_FRAME_TIME - elapsed));
            }
            self.last_update_time = Instant::now();
        }
    }
}






