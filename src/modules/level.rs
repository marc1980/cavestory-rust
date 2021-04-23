extern crate serde;
extern crate quick_xml;

use std::{fs::File, io::Read, path::Path};
use super::SpriteSheet;
use sdl2::rect::Rect;
use serde::Deserialize;
use quick_xml::de::{from_str};
use crate::modules::{Rectangle, Point, Slope};

pub struct Level {
    map: Map,
}
#[derive(Debug, Deserialize, PartialEq)]
struct Map {
   // version: &str,
   // orientation: &str,
    width: i32,
    height: i32,
    tileheight: i32,
    #[serde(rename = "tileset", default)]
    tilesets: Vec<Tileset>,
    #[serde(rename = "layer", default)]
    layers: Vec<Layer>,
    #[serde(rename = "objectgroup", default)]
    objectgroups: Vec<Objectgroup>,
}
#[derive(Debug, Deserialize, PartialEq)]
struct Tileset {
    firstgid: i32, //="1" 
    name: SpriteSheet, //="PrtCave" 
    tilewidth: i32, //="16" 
    tileheight: i32, //="16"
    image: Image,
}
#[derive(Debug, Deserialize, PartialEq)]
struct Image {
    source: String, //="../content/tilesets/PrtCave.png" 
    width: i32, //="256" 
    height: i32, //="80"/>
}
#[derive(Debug, Deserialize, PartialEq)]
pub struct Layer {
    name: String, //="background" 
    width: i32, //="20" 
    height: i32, //="16"
    data: Data,
}
#[derive(Debug, Deserialize, PartialEq)]
struct Data {
    #[serde(rename = "tile", default)]
    tile: Vec<Tile>
}

#[derive(Debug, Deserialize, PartialEq)]
struct Tile {
    gid: u32
}

#[derive(Debug, Deserialize, PartialEq)]
struct Objectgroup {
    name: String,
    #[serde(rename = "object", default)]
    objects: Vec<Object>
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Object {
    id: u32,
    x: f32,
    y: f32,
    width: Option<f32>,
    height: Option<f32>,
    polyline: Option<Polyline>
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Polyline {
    points: String
}

pub struct TileMap {
    pub tiles_texture: SpriteSheet,
    pub source: Vec<Rect>,
    pub dest: Vec<Rect>,
}

#[derive(Debug)]
pub struct Collisions {
    pub blocks: Vec<Rectangle>,
    pub slopes: Vec<Slope>
}

impl Level {
    pub fn new() -> Level {
        let path = Path::new("./content/maps/Map1.tmx");
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(err) => panic!("file error: {}", err)
        };
        let mut xml = String::new();
        match file.read_to_string(&mut xml) {
            Ok(x) => x,
            Err(err) => panic!("read error: {}", err)
        };
        let map: Map  = match from_str(&xml) {
            Ok(m) => m,
            Err(err) => panic!("deserialize error: {}", err)
        };
        return Level {
            map
        }
    }

    pub fn get_tile_map(&self) -> TileMap {
        let mut source = Vec::new();
        let mut dest = Vec::new();

        let tiles_source: u32 = 16;
        let tiles_destination: u32 = 20;
        let tile_size_source_px: u32 = 16;
        let tile_size_dest_px: u32 = 32;
        let tiles_texture = self.map.tilesets.first().unwrap().name;

        for layer in self.map.layers.iter() {
            for (idx, tile) in layer.data.tile.iter().enumerate() {
                let gid = tile.gid; 
                if gid > 0 {
                    let source_x = (gid-1) % tiles_source * tile_size_source_px;
                    let source_y = (gid-1) / tiles_source * tile_size_source_px;
                    let dest_x = idx as u32 % tiles_destination * tile_size_dest_px;
                    let dest_y = idx as u32 / tiles_destination * tile_size_dest_px;
                    let source_rect = Rect::new(source_x as i32, source_y as i32, tile_size_source_px, tile_size_source_px);
                    let dest_rect = Rect::new(dest_x as i32, dest_y as i32, tile_size_dest_px, tile_size_dest_px);
                    source.push(source_rect);
                    dest.push(dest_rect);
                }
            }
        }
        TileMap {
            tiles_texture,
            source,
            dest
        }
    }

    pub fn get_spawn_point(&self, scale: f32) -> (f32, f32) {
        let objects: &Vec<Object> = &self.map.objectgroups
            .iter()
            .find(|og| og.name == "spawn points")
            .expect("spawn points not found in level")
            .objects;
        let spawn_point_object = objects.first().expect("spawn point not found");

        return ((spawn_point_object.x * scale), (spawn_point_object.y * scale))
    }

    pub fn get_collisions(&self, scale: f32) -> Collisions {
        Collisions {
            blocks:  self.get_blocks(scale),
            slopes: self.get_slopes(scale)
        }
    }

    pub fn get_blocks(&self, scale: f32) -> Vec<Rectangle> {
        let mut blocks = Vec::new();

        for object in self.map.objectgroups
            .iter()
            .find(|og| og.name == "collisions")
            .expect("collision blocks not found in level file")
            .objects
            .iter() {
                blocks.push(Rectangle {
                    x: object.x * scale,
                    y: object.y * scale,
                    width: object.width.expect("collision block: no width") * scale,
                    height: object.height.expect("collision block: no height") * scale
                });
        }
        return  blocks;
    }

    pub fn get_slopes(&self, scale: f32) -> Vec<Slope> {
        let mut slopes = Vec::new();

        for object in self.map.objectgroups
            .iter()
            .find(|og| og.name == "slopes")
            .expect("slopes not found in level file")
            .objects
            .iter() {
            let p2: Vec<f32> = object.polyline.as_ref()
                .expect("slope: no polyline found")
                .points
                .split(|c| c == ' ' || c == ',')
                .map(|s| s.parse::<f32>()
                .expect("polyline coordinate is not a valid f32"))
                .filter(|i| *i != 0f32)
                .collect();

            slopes.push(Slope {
                p1: Point {
                    x: object.x * scale,
                    y: object.y * scale
                },
                // the second coordinate (and subsequent ones) is relative to the on in the object
                p2: Point {
                    x: (object.x + p2[0]) * scale,
                    y: (object.y + p2[1]) * scale
                },
            });
        }
        return  slopes;
    }
}

