use sdl2::render::Texture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

pub struct Cube {
    pub texture: Texture,
}

impl Cube {
    pub fn new() -> Self {
        let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (256,256)).unwrap();
        
        Cube { texture, }
    }
    
    pub fn draw() {
        todo!();
    }
}