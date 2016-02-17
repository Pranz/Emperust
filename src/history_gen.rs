
use map::Map;
use settings::Settings;
use biome::{Biome, BiomeType};

use rand::{sample, thread_rng};
use std::collections::{HashMap, HashSet};

pub struct Language(pub usize);

pub struct City {
    pub language: Language,
    pub population: u32,
}

pub fn place_initial_cities(settings: &Settings, map: &Map) -> HashMap<(usize, usize), City> {
    let mut cities = HashMap::new();
    let mut id_gen = (0..100000);
    let valid_city_locations = map.get_coordinate_iter()
        .filter(|&(x,y)| map.get_biome(x,y).category() != BiomeType::Water &&
                map.get_biome(x,y) == Biome::Woodlands &&
                map.neighbour_positions(x,y).iter()
                .any(|&(x,y)| map.get_biome(x,y).category() == BiomeType::Water));
    println!("cities placed: {}", sample(&mut thread_rng(),
           valid_city_locations,
           settings.initial_cities as usize)
        .into_iter()
             .map(|location| cities.insert(location, City {
                 language: Language(id_gen.next().unwrap()),
                 population: 100,
             }))
        .count());
    return cities;
}

pub fn generate_botanical_set_map(biome_map: &[Biome], height_map: &[u8],
                                  width: usize, height: usize) {
    return;
}
