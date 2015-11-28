

use tcod::noise::{Noise, NoiseType};
use tcod::{Color, colors, chars};

use num::pow;
use itertools::Product;
use std::sync::mpsc::Sender;
use rand::{thread_rng, sample};

use point::Point;
use settings::Settings;
use biome::{Biome, BiomeType, BiomeRepresentation};
use world_gen::{get_noise_map, combine_scalar_fields, get_distance_map, get_distance_vertical_map};
use game::ProgressInfo;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    pub position: Point<usize>,
    pub height: u8,
    pub temperature: u8,
    pub rainfall: u8,
    pub biome: Biome,
}

impl Tile {
    pub fn graphical_representation(self, map: &Map) -> (char, Color, Color) {
        match self.biome.graphical_representation() {
            BiomeRepresentation::Standard(chr, fg, bg) => (chr, fg, bg),
            BiomeRepresentation::Ocean =>
                (' ', colors::WHITE, Color::new(0,0, ((self.height as f32 - 25.0) * 2.0) as u8)),
            BiomeRepresentation::Mountain => {
                ('^', if self.height > 182 { colors::LIGHTEST_GREY }
                 else if self.height > 175 { colors::LIGHTER_GREY }
                 else { colors::LIGHT_GREY }, colors::DARK_GREY)
            },
            BiomeRepresentation::River => {
                let neighbour_rivers: Vec<(isize, isize)> = map
                    .neighbour_positions(self.position.x, self.position.y)
                    .into_iter()
                    .filter(|&(x,y)|
                            map.get_biome(x,y) == Biome::River)
                    .map(|(x,y)|
                         (self.position.x as isize - x as isize,
                          self.position.y as isize - y as isize))
                    .collect();

                let chr = if neighbour_rivers.contains(&(1, 0)) &&
                    neighbour_rivers.contains(&(0, 1))
                { chars::SE }
                else if neighbour_rivers.contains(&(1, 0)) &&
                    neighbour_rivers.contains(&(0, -1))
                { chars::NE }
                else if neighbour_rivers.contains(&(-1, 0)) &&
                    neighbour_rivers.contains(&(0, 1))
                { chars::SW }
                else if neighbour_rivers.contains(&(-1, 0)) &&
                    neighbour_rivers.contains(&(0, -1))
                { chars::NW }
                else if neighbour_rivers.contains(&(-1, 0)) &&
                    neighbour_rivers.contains(&(1, 0))
                { chars::HLINE }
                else if neighbour_rivers.contains(&(0, -1)) &&
                    neighbour_rivers.contains(&(0, 1))
                { chars::VLINE }
                else { '+' };
                
                (chr, colors::LIGHTER_BLUE, colors::BLUE)
            },
        }
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
               rivers: usize,
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
        
        let mut map = Map {
            height_map: heights,
            biome_map: biomes,
            temperature_map: temperature_map,
            rainfall_map: rainfall_map,
            width: width,
            height: height,
        };
        map.create_rivers(rivers);
        map
    }

    pub fn create_rivers(&mut self, amount: usize) -> Vec<Vec<(usize, usize)>> {
        let random_points = sample(
            &mut thread_rng(),
            iproduct!(0..self.width, 0..self.height),
            amount * 10);
        random_points.into_iter().map(|(x,y)| {
            let (highest_x, highest_y) = iproduct!(0..10, 0..10)
                .map(|(offset_x, offset_y)|
                     (x + offset_x, y + offset_y))
                .filter(|&(xx, yy)|
                        self.in_bounds(xx,yy))
                .take(amount)
                .max_by(|&(xx, yy)|
                        self.get_height(xx,yy))
                .unwrap();
            
            self.create_river(highest_x, highest_y)
        }).collect()
    }

    pub fn create_river(&mut self, x_orig: usize, y_orig: usize) -> Vec<(usize, usize)> {
        if self.get_biome(x_orig,y_orig).category() == BiomeType::Water {
            return vec![];
        }
        
        let mut nodes = vec![(x_orig, y_orig)];
        self.set_biome(x_orig, y_orig, Biome::River);
        let (mut x, mut y) = (x_orig as isize, y_orig as isize);
        let mut rng = thread_rng();
        
        while nodes.len() < 200 {
            let potential_nodes : Vec<(isize, isize)> =
                vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .into_iter()
                .filter(|&t| {
                    let (xx,yy) = t;
                    self.in_bounds_isize(xx as isize,yy as isize) &&
                        self.get_height(xx as usize, yy as usize) <
                        (self.get_height(x as usize, y as usize) + 2) &&
                        self.neighbour_positions(xx as usize, yy as usize)
                        .into_iter()
                        .filter(|&t|
                                self.get_biome(t.0,t.1) == Biome::River)
                        .count() < 2
                }).collect();

            // If it is empty, returns nodes.
            // If any neighbour is water, returns nodes
            if potential_nodes.len() == 0 ||
                potential_nodes.iter().any(|&(xx,yy)| {
                    let biome = self.get_biome(xx as usize, yy as usize);
                    biome == Biome::Ocean
                })
            { return nodes; }

            let random_node = sample(&mut rng, potential_nodes.iter(), 1)[0];
            
            x = random_node.0 as isize;
            y = random_node.1 as isize;
            self.set_biome(x as usize, y as usize, Biome::River);
            nodes.push((x as usize, y as usize));
        }
        nodes
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn neighbour_positions(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let (x,y) = (x as isize, y as isize);
        let neighbours: Vec<(usize, usize)> =
            vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            .filter(|&t|{
                let (xx,yy) = t;
                self.in_bounds_isize(xx,yy)
            })
            .map(|(xx,yy)|
                 (xx as usize, yy as usize))
            .collect();
        neighbours
    }

    pub fn in_bounds_isize(&self, x: isize, y: isize) -> bool {
        x < self.width as isize && y < self.height as isize
            && x >= 0 && y >= 0
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        Tile {
            position: Point::new(x,y),
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

    pub fn set_biome(&mut self, x: usize, y: usize, biome: Biome) {
        self.biome_map[x * self.height + y] = biome;
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
                                   settings.height_map_coefficient * 9.0);
    let map_width = settings.map_width;
    let map_height = settings.map_height;
    let distance_map = get_distance_map(map_width as f32, map_height as f32);
    
    let height_map = combine_scalar_fields(vec![(noise_gen, 0.75),
                                                (turbulence, 0.25),
                                                (distance_map, 0.0)]);
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
