
//
// Utilities for world generation
//

use tcod::noise::{NoiseInitializer, Noise, NoiseType};

use num::pow;

const NOISE_TYPE : NoiseType = NoiseType::Perlin;

//
//returns a noise function which gives numbers between 0.0 and 1.0
//

pub type ScalarField = Box<Fn(f32, f32) -> f32>;

pub fn get_noise_map(lacunarity: f32, hurst: f32, coefficient: f32) -> ScalarField {
    let noise_map = Noise::init_with_dimensions(2)
        .noise_type(NOISE_TYPE)
        .lacunarity(lacunarity)
        .hurst(hurst)
        .init();
    Box::new(move |x: f32, y: f32| {
        (1.0 + noise_map.get([x * coefficient, y * coefficient])) / 2.0
    })
}

pub fn combine_scalar_fields(scalar_fields: Vec<(ScalarField, f32)>) -> ScalarField {
    Box::new(move |x, y| {
        let mut acc: f32 = 0.0;
        for &(ref f, ref k) in scalar_fields.iter() {
            acc += (*f)(x,y) * k;
        }
        acc
    })
}

pub fn get_distance_map(map_width: f32, map_height: f32) -> ScalarField {
    let max_distance = (pow(map_width / 2.0, 2) +
                        pow(map_height / 2.0, 2)).sqrt();
    Box::new(move |x, y| {
        1.0 - ((pow(x - map_width / 2.0, 2) +
          pow(y - map_height / 2.0, 2))
         .sqrt() / max_distance)
    })
}

pub fn get_distance_vertical_map(map_height: f32) -> ScalarField {
    let center_y = map_height / 2.0;
    Box::new(move |_, y| { 1.0 - ((y - center_y).abs() / center_y)})
}


