use sdl2::{rect::Rect};

pub struct Sprite<'a> {
	pub source_rect: Rect,
	pub sprite_sheet: &'a str,
	// bounding_box: Rectangle,
	// x: i32, 
    // y: i32
}

impl Sprite<'_> {

    pub fn new( filepath: &str, sourceX: i32, sourceY: i32, width: u32, height: u32,
        posX: i32, posY: i32) -> Sprite {

        
        Sprite{
            source_rect: Rect::new(sourceX, sourceY, width, height) ,
            sprite_sheet: filepath,
        }
    }


}

