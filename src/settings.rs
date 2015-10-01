
use yaml_rust::YamlLoader;

use std::fs::File;
use std::io::prelude::*;
use std::io;

pub struct Settings {
    pub font_path: String,
    pub title: String,
    pub main_window_width: i32,
    pub main_window_height: i32,
    pub map_window_width: usize,
    pub map_window_height: usize,
    pub map_width: usize,
    pub map_height: usize,
    pub zoomed_map_width: usize,
    pub zoomed_map_height: usize,
    pub debug_console_width: usize,
    pub debug_console_height: usize,
    pub height_map_coefficient: f32,
    pub height_map_lacunarity: f32,
    pub height_map_hurst: f32,
    pub tree_line: u8,
    pub ocean_line: u8,
    pub temperature_y_dependence: f32,
    pub temperature_turbulence: f32,
    pub temperature_turbulence_dependence: f32,
}

impl Settings {
    pub fn read(path: &'static str) -> io::Result<Settings> {
        let mut contents = String::new();
        let mut f = try!(File::open(path));
        try!(f.read_to_string(&mut contents));

        //only get the first document
        let ref doc = YamlLoader::load_from_str(&contents)
            .unwrap()[0];
        Ok(Settings {
            font_path: doc["font_path"].as_str().unwrap().to_string(),
            title: doc["title"].as_str().unwrap().to_string(),
            main_window_width: doc["main_window"]["width"]
                .as_i64().unwrap() as i32,
            main_window_height: doc["main_window"]["height"]
                .as_i64().unwrap() as i32,
            map_width: doc["map"]["width"].as_i64().unwrap() as usize,
            map_height: doc["map"]["height"].as_i64().unwrap() as usize,
            map_window_width: doc["map_window"]["width"].as_i64().unwrap() as usize,
            map_window_height: doc["map_window"]["height"].as_i64().unwrap() as usize,
            zoomed_map_width: doc["zoomed_map_window"]["width"]
                .as_i64().unwrap() as usize,
            zoomed_map_height: doc["zoomed_map_window"]["height"]
                .as_i64().unwrap() as usize,
            debug_console_width: doc["debug_window"]["width"]
                .as_i64().unwrap() as usize,
            debug_console_height: doc["debug_window"]["width"]
                .as_i64().unwrap() as usize,
            height_map_coefficient: doc["height_map"]["coefficient"]
                .as_f64().unwrap() as f32,
            height_map_lacunarity: doc["height_map"]["lacunarity"]
                .as_f64().unwrap() as f32,
            height_map_hurst: doc["height_map"]["hurst"]
                .as_f64().unwrap() as f32,
            tree_line: doc["world_gen"]["tree_line"].as_i64().unwrap() as u8,
            ocean_line: doc["world_gen"]["ocean_line"].as_i64().unwrap() as u8,
            temperature_turbulence_dependence: doc["temperature"]["turbulence_dependence"]
                .as_f64().unwrap() as f32,
            temperature_y_dependence: doc["temperature"]["y_dependence"]
                .as_f64().unwrap() as f32,
            temperature_turbulence: doc["temperature"]["y_dependence"]
                .as_f64().unwrap() as f32,
        })
    }
}
