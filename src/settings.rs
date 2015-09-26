
use yaml_rust::YamlLoader;

use std::fs::File;
use std::io::prelude::*;
use std::io;

pub struct Settings {
    pub font_path: String,
    pub title: String,
    pub width: i32,
    pub height: i32,
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
            width: doc["main_window"]["width"].as_i64().unwrap() as i32,
            height: doc["main_window"]["height"].as_i64().unwrap() as i32,
        })
    }
}
