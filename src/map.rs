
use tcod::noise::{Noise, NoiseType};

use settings::Settings;

type Tile = u8;

pub struct Map {
    tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new<F>(width: usize,
                  height: usize,
                  height_map: F)
                  -> Map
    where F: Fn(usize, usize) -> u8 {
        let mut tiles: Vec<Tile> = Vec::new();
        
        for x in (0..width) {
            for y in (0..height) {
                tiles.push(height_map(x,y));
            }
        }
        
        Map {
            tiles: tiles,
            width: width,
            height: height,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        assert!(x >= 0 &&
                x < self.width &&
                y >= 0 &&
                y < self.height);
        self.tiles[x + y * self.width]
    }
}

//
// Generates a height map using Settings
//
pub fn get_height_map(settings: &Settings) -> Box<Fn(usize, usize) -> u8> {
    let k = settings.height_map_coefficient;
    let noise_gen = Noise::init_with_dimensions(2)
        .noise_type(NoiseType::Simplex)
        .lacunarity(settings.height_map_lacunarity)
        .hurst(settings.height_map_hurst)
        .init();
    Box::new( move |x: usize, y: usize| {
        let (x, y) = (x as f32, y as f32);
        ((noise_gen.get(([x * k, y * k])) + 1f32) * 128f32) as u8
    })
}
