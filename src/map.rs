

use tcod::noise::{Noise, NoiseType};
use tcod::{Color, colors};

use num::pow;
use itertools::Product;

use settings::Settings;
use biome::Biome;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    pub height: u8,
    pub biome: Biome,
}

impl Tile {
    pub fn graphical_representation(self) -> (char, Color, Color) {
        match self.biome {
            Biome::Ocean => (' ', colors::WHITE, Color::new(0,0, ((self.height as f32 - 25.0) * 2.0) as u8)),
            Biome::Plains => (',', colors::DARKER_CHARTREUSE, colors::DARK_CHARTREUSE),
            Biome::Mountain => ('^', colors::GREY, colors::DARK_GREY),
        }
    }
}

pub struct Map {
    tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new<F>(width: usize,
                  height: usize,
                  ocean_line: u8,
                  tree_line: u8,
                  height_map: F)
                  -> Map
    where F: Fn(usize, usize) -> u8 {
        let mut tiles: Vec<Tile> = Vec::new();
        
        for x in (0..width) {
            for y in (0..height) {
                let height = height_map(x,y);
                tiles.push(Tile {
                    height: height,
                    biome: if height < ocean_line
                    { Biome::Ocean } else if height < tree_line
                    { Biome::Plains } else
                    { Biome::Mountain }
                });
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
        self.tiles[x * self.height + y]
    }
}

//
// Generates a height map using Settings
//
pub fn get_height_map(settings: &Settings) -> Box<Fn(usize, usize) -> u8> {
    let k = settings.height_map_coefficient;
    let noise_gen = Noise::init_with_dimensions(2)
        .noise_type(NoiseType::Perlin)
        .lacunarity(settings.height_map_lacunarity)
        .hurst(settings.height_map_hurst)
        .init();
    let noise_gen2 = Noise::init_with_dimensions(2)
        .noise_type(NoiseType::Perlin)
        .lacunarity(settings.height_map_lacunarity)
        .hurst(settings.height_map_hurst)
        .init();
    let map_width = settings.map_width;
    let map_height = settings.map_height;
    let max_distance = (pow(map_width as f32 / 2.0, 2) +
                        pow(map_height as f32 / 2.0, 2)).sqrt();
    Box::new(move |x: usize, y: usize| {
        let (x, y) = (x as f32, y as f32);
        let height = ((noise_gen.get(([x * k, y * k])) + 1f32) * 128f32) as u8;
        let height2 = ((noise_gen2.get(([x * 0.001, y * 0.001])) + 1f32) * 128f32) as u8;
        let distance = (pow(x - map_width as f32 / 2.0, 2) +
                        pow(y - map_height as f32 / 2.0, 2))
            .sqrt() / max_distance;
        255 - ((((height as u16 + height2 as u16) as f32 / 2.0) as f32 / 1.5) as u8 + (distance * 85.0) as u8)
    })
}

pub fn zoomed_map(map: &Map, width: usize, height: usize, settings: &Settings) -> Map {
    let (ratioX, ratioY, remainderX, remainderY) = (map.width / width,
                                                    map.height / height,
                                                    map.width % width,
                                                    map.height % height);
    let mut tiles = Vec::new();
    let (ocean_line, tree_line) = (settings.ocean_line, settings.tree_line);
    
    for (x,y) in Product::new((0..width),(0..height)) {
        let height = Product::new(((x*ratioX)..((x+1)*ratioX)), ((y*ratioY)..((y+1)*ratioY))).map(|(xx,yy)| {
            map.get_tile(xx,yy).height
        }).fold(0, |a,b| a as u64 + b as u64) / (ratioX as u64 * ratioY as u64);
        tiles.push(Tile {
            height: (height as u8),
            biome: if (height as u8) < ocean_line
            { Biome::Ocean } else if (height as u8) < tree_line
            { Biome::Plains } else
            { Biome::Mountain }
        });
    }
    Map {
        tiles: tiles,
        width: width,
        height: height,
    }
    
}
