extern crate serde;
extern crate quick_xml;

use std::{fs::File, io::Read, path::Path};

use serde::Deserialize;
use quick_xml::de::{from_str, DeError};


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
   // objectgroup: Objectgroup,
}
#[derive(Debug, Deserialize, PartialEq)]
struct Tileset {
    firstgid: i32, //="1" 
    name: String, //="PrtCave" 
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
struct Layer {
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
    gid: i32
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
}

