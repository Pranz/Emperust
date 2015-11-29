
use map::Map;
use settings::Settings;
use biome::{Biome, BiomeType};

use rand::{sample, thread_rng};
use std::collections::{HashMap, HashSet};

pub struct City {
    void: (),
}

pub fn place_initial_cities(settings: &Settings, map: &Map) -> HashSet<(usize, usize)> {
    let mut cities = HashSet::new();
    let valid_city_locations = map.get_coordinate_iter()
        .filter(|&(x,y)| map.get_biome(x,y).category() != BiomeType::Water &&
                map.get_biome(x,y) == Biome::Woodlands &&
                map.neighbour_positions(x,y).iter()
                .any(|&(x,y)| map.get_biome(x,y) == Biome::River));
    println!("cities placed: {}", sample(&mut thread_rng(),
           valid_city_locations,
           settings.initial_cities as usize)
        .into_iter()
        .map(|location| cities.insert(location))
        .count());
    return cities;
}
