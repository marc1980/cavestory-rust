extern crate sdl2;
use sdl2::{EventPump, event::Event, image::LoadTexture, rect::Rect};
use sdl2::keyboard::Keycode;
use specs::prelude::*;
use std::{collections::HashMap, thread, time::{Duration, Instant}};
use components::{Sprite, SpriteSheet};
use super::{Animation, Graphics, Input, Level, Movement, Physics, Player, PlayerStatus, Position, components};
use crate::modules::{CollisionTargets, Collision, Rectangle};

const MAX_FPS: u64 = 20;
const MAX_FRAME_TIME: u64 = 1000 / MAX_FPS;
const BLOCK_SIZE_PX: u32 = 16;
const SCALE: f32 = 2.0;

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
        // let col = self.level.get_collisions();

        let texture_creator = self.graphics.canvas.texture_creator();
        let mut spritesheets =  HashMap::new();
        spritesheets.insert(
            SpriteSheet::MyChar,
            texture_creator.load_texture("./content/sprites/MyChar.png").unwrap()
        );
        spritesheets.insert(
            SpriteSheet::PrtCave,
            texture_creator.load_texture("./content/sprites/PrtCave.png").unwrap()
        );

        let mut world = World::new();
        world.register::<Player>();
        world.register::<Position>();
        world.register::<Movement>();
        world.register::<CollisionTargets>();
        world.register::<Sprite>();
    
        let player = world.create_entity()
            .with(Player {
                animation_frame: 0
            })
            .with(Position {
                rect: Rectangle {
                    x: 320.0, y: 260.0,
                    width: BLOCK_SIZE_PX as f32 * SCALE,
                    height: BLOCK_SIZE_PX as f32 * SCALE
                }})
            .with(Movement {
                speed: 0, direction: PlayerStatus::Stopped, animation_frame: 0
            })
            .with(CollisionTargets {
                collisions: self.level.get_collisions(2.0)
            })
            .with(Sprite {
                spritesheet: SpriteSheet::MyChar,
                source_rect: Rect::new(0, 0, BLOCK_SIZE_PX, BLOCK_SIZE_PX)
            })
            .build();

        let mut dispatcher = DispatcherBuilder::new()
            .with(Input, "input", &[])
            .with(Physics, "physics", &["input"])
            .with(Collision, "collision", &["physics"])
            .with(Animation, "animation", &["input"])
            .build();

        dispatcher.setup(&mut world);       
        
        let mut event_pump: EventPump =
            self.graphics.sdl_context.event_pump().expect("failed to load event pump");
        
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
    
            let player_rect = world.read_component::<Position>().get(player).unwrap().rect;
            self.graphics.render(world.read_component::<Sprite>().get(player).unwrap(),
                player_rect, &self.level.get_tile_map(), &spritesheets);
            
            let elapsed = self.last_update_time.elapsed().as_millis() as u64;

            if elapsed < MAX_FRAME_TIME {
                thread::sleep(Duration::from_millis(MAX_FRAME_TIME - elapsed));
            }
            self.last_update_time = Instant::now();
        }
    }
}






