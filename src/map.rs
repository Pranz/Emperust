

use tcod::noise::{Noise, NoiseType};
use tcod::{Color, colors};

use num::pow;
use itertools::Product;
use std::sync::mpsc::Sender;

use settings::Settings;
use biome::Biome;
use world_gen::{get_noise_map, combine_scalar_fields, get_distance_map, get_distance_vertical_map};
use game::ProgressInfo;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    pub height: u8,
    pub temperature: u8,
    pub rainfall: u8,
    pub biome: Biome,
}

impl Tile {
    pub fn graphical_representation(self) -> (char, Color, Color) {
        self.biome.graphical_representation(self.height)
    }
}

pub struct Map {
    pub height_map: Vec<u8>,
    pub biome_map: Vec<Biome>,
    pub temperature_map: DiscreteField,
    pub rainfall_map: Box<Fn(usize, usize, u8) -> u8>,
    pub width: usize,
    pub height: usize,
}

pub struct ZoomedMap {
    pub biome_map: Vec<Biome>,
    pub width: usize,
    pub height: usize,
}

pub type DiscreteField = Box<Fn(usize, usize) -> u8>;

impl Map {
    pub fn new(width: usize,
               height: usize,
               ocean_line: u8,
               tree_line: u8,
               height_map: DiscreteField,
               temperature_map: DiscreteField,
               rainfall_map: Box<Fn(usize, usize, u8) -> u8>,
               tx: Option<&Sender<ProgressInfo>>)
               -> Map
    {
        let mut heights: Vec<u8> = Vec::new();
        let mut biomes: Vec<Biome> = Vec::new();
        
        for x in (0..width) {
            tx.as_ref().map(|s| s.send(ProgressInfo::FinishedColumn(x)));
            for y in (0..height) {
                let height = height_map(x,y);
                heights.push(height);
                biomes.push(Biome::new(height, temperature_map(x,y), rainfall_map(x,y, height),
                                       tree_line, ocean_line));
            }
        }

        tx.map(|s| s.send(ProgressInfo::Done));
        
        Map {
            height_map: heights,
            biome_map: biomes,
            temperature_map: temperature_map,
            rainfall_map: rainfall_map,
            width: width,
            height: height,
        }
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        Tile {
            height: self.height_map[x * self.height + y],
            rainfall: (*self.rainfall_map)(x, y, self.height_map[x * self.height + y]),
            temperature: (*self.temperature_map)(x,y),
            biome: self.biome_map[x * self.height + y],
        }
    }

    #[inline(always)]
    pub fn get_height(&self, x: usize, y: usize) -> u8 {
        self.height_map[x * self.height + y]
    }

    #[inline(always)]
    pub fn get_biome(&self, x: usize, y: usize) -> Biome {
        self.biome_map[x * self.height + y]
    }
}

impl ZoomedMap {
    pub fn get_biome(&self, x: usize, y: usize) -> Biome {
        self.biome_map[x * self.height + y]
    }
}

//
// Generates a height map using Settings
//
pub fn get_height_map(settings: &Settings) -> Box<Fn(usize, usize) -> u8> {
    let noise_gen = get_noise_map(settings.height_map_lacunarity,
                                  settings.height_map_hurst,
                                  settings.height_map_coefficient);
    let turbulence = get_noise_map(settings.height_map_lacunarity,
                                   settings.height_map_hurst,
                                   settings.height_map_coefficient * 6.0);
    let map_width = settings.map_width;
    let map_height = settings.map_height;
    let distance_map = get_distance_map(map_width as f32, map_height as f32);
    
    let height_map = combine_scalar_fields(vec![(noise_gen, 0.65),
                                                (turbulence, 0.15),
                                                (distance_map, 0.2)]);
    Box::new(move |x: usize, y: usize| {
        let (x, y) = (x as f32, y as f32);
        (height_map(x,y) * 255.0) as u8
    })
}

pub fn get_temperature_map(settings: &Settings) -> DiscreteField {
    let noise_gen = get_noise_map(settings.height_map_lacunarity,
                                  settings.height_map_hurst,
                                  settings.temperature_coefficient);
    let turbulence = get_noise_map(settings.height_map_lacunarity,
                                   settings.height_map_hurst,
                                   settings.temperature_coefficient *
                                   settings.temperature_turbulence);
    let distance_map = get_distance_vertical_map(settings.map_height as f32);
    let temperature_map = combine_scalar_fields(
        vec![(turbulence, settings.temperature_turbulence_dependence),
             (distance_map, settings.temperature_y_dependence),
             (noise_gen, 1.0 - settings.temperature_y_dependence - settings.temperature_turbulence_dependence)]);
    Box::new(move |x: usize, y: usize| {
        let (x,y) = (x as f32, y as f32);
        (temperature_map(x,y) * 255.0) as u8
    })
}

pub fn get_rainfall_map(settings: &Settings) -> Box<Fn(usize, usize, u8) -> u8> {
    let noise_gen = get_noise_map(settings.height_map_lacunarity,
                                  settings.height_map_hurst,
                                  settings.rainfall_coefficient);
    let turbulence = get_noise_map(settings.height_map_lacunarity,
                                   settings.height_map_hurst,
                                   settings.rainfall_coefficient *
                                   settings.rainfall_turbulence);
    let rainfall_map = combine_scalar_fields(
        vec![(turbulence, settings.rainfall_turbulence_dependence),
             (noise_gen, 1.0 - settings.rainfall_turbulence_dependence)]);

    let height_dependence = settings.rainfall_height_dependence;
    
    Box::new( move |x, y, h| {
        (h as f32 * height_dependence) as u8 + (rainfall_map(x as f32, y as f32) *
                                                (1.0 - height_dependence) * 255.0) as u8
    })
}

pub fn zoomed_map(map: &Map, width: usize, height: usize, settings: &Settings) -> ZoomedMap {
    let (ratioX, ratioY, remainderX, remainderY) = (map.width / width,
                                                    map.height / height,
                                                    map.width % width,
                                                    map.height % height);
    let mut biome_map = Vec::new();
    let (ocean_line, tree_line) = (settings.ocean_line, settings.tree_line);
    
    for (x,y) in Product::new((0..width),(0..height)) {
        let height = Product::new(((x*ratioX)..((x+1)*ratioX)), ((y*ratioY)..((y+1)*ratioY))).map(|(xx,yy)| {
            map.get_height(xx,yy)
        }).fold(0, |a,b| a as u64 + b as u64) / (ratioX as u64 * ratioY as u64);
        let temperature = Product::new(((x*ratioX)..((x+1)*ratioX)), ((y*ratioY)..((y+1)*ratioY))).map(|(xx,yy)| {
            (*map.temperature_map)(xx,yy)
        }).fold(0, |a,b| a as u64 + b as u64) / (ratioX as u64 * ratioY as u64);
        
        let rainfall = Product::new(((x*ratioX)..((x+1)*ratioX)), ((y*ratioY)..((y+1)*ratioY))).map(|(xx,yy)| {
            (*map.rainfall_map)(xx,yy, height as u8)
        }).fold(0, |a,b| a as u64 + b as u64) / (ratioX as u64 * ratioY as u64);
        
        biome_map.push(Biome::new(height as u8, temperature as u8, rainfall as u8, tree_line, ocean_line));
    }
    ZoomedMap {
        biome_map: biome_map,
        width: width,
        height: height,
    }
    
}
