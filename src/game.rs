
use tcod::console::{Console, Offscreen};

use settings::Settings;
use input::UserCommand;
use point::Point;
use map::{Map, ZoomedMap, get_height_map, get_temperature_map, get_rainfall_map, zoomed_map};
use direction::Direction;
use history_gen::{City, place_initial_cities};

use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};
use std::sync::mpsc::Sender;

const CAMERA_STRICTNESS : usize = 24;

pub struct Game {
    pub map: Map,
    pub zoomed_map: ZoomedMap,
    pub cursor: Point<i32>,
    pub camera: Point<i32>,
    pub settings: Settings,
    pub cities: HashMap<(usize, usize), City>,
    pub map_console: Offscreen,
    pub zoomed_map_console: Offscreen,
    pub debug_console: Offscreen,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ProgressInfo {
    FinishedColumn(usize),
    Done
}

impl Game {
    pub fn new(settings: Settings, tx: Sender<ProgressInfo>) -> Game {
        let height_map = get_height_map(&settings);
        let rainfall_map = get_rainfall_map(&settings);
        let temperature_map = get_temperature_map(&settings);
        let map : Map = Map::new(settings.map_width as usize,
                                 settings.map_height as usize,
                                 settings.ocean_line,
                                 settings.tree_line,
                                 settings.river_amount,      
                                 height_map,
                                 temperature_map,
                                 rainfall_map,
                                 Some(&tx));
        Game {
            zoomed_map: zoomed_map(&map,
                                   settings.zoomed_map_width,
                                   settings.zoomed_map_height,
                                   &settings),
            cities: place_initial_cities(&settings, &map),
            map: map,
            cursor: Point::new(settings.map_height as i32 / 2, settings.map_width as i32 / 2),
            camera: Point::new(0, 0),
            map_console: Offscreen::new(settings.map_window_width as i32,
                                        settings.map_window_height as i32),
            zoomed_map_console: Offscreen::new(settings.zoomed_map_width as i32,
                                               settings.zoomed_map_height as i32),
            debug_console: Offscreen::new(settings.debug_console_width as i32,
                                          settings.debug_console_height as i32),
            settings: settings,
        }
    }

    pub fn execute_command(&mut self, cmd: UserCommand) {

        
        match cmd {
            UserCommand::Move(dir, shift_is_pressed) => {
                self.move_cursor(dir.to_point().map(|x| match shift_is_pressed {
                    true => x * 5,
                    false => x,
                }));
            }
            UserCommand::RegenMap => { self.regenerate_map(); }
            UserCommand::CreateRiver => {
                self.map.create_river(self.cursor.x as usize, self.cursor.y as usize);
            }
            _ => {},
        }
    }

    pub fn regenerate_map(&mut self) {
        let height_map = get_height_map(&self.settings);
        let temperature_map = get_temperature_map(&self.settings);
        let rainfall_map = get_rainfall_map(&self.settings);
        self.map = Map::new(self.settings.map_width as usize,
                            self.settings.map_width as usize,
                            self.settings.ocean_line,
                            self.settings.tree_line,
                            self.settings.river_amount,
                            height_map,
                            temperature_map,
                            rainfall_map,
                            None);
        self.zoomed_map = zoomed_map(&self.map, self.settings.zoomed_map_width,
                                     self.settings.zoomed_map_height, &self.settings);
    }

    pub fn move_cursor(&mut self, dpos: Point<i32>) {
        let Point {x: dx, y: dy} = dpos;
        self.cursor.x = max(0, min(self.map.width as i32 - 1,
                                   self.cursor.x + dx));
        self.cursor.y = max(0, min(self.map.height as i32 - 1,
                                   self.cursor.y + dy));
        let camera_center = Point {
            x: self.camera.x + self.map_console.width() as i32 / 2,
            y: self.camera.y + self.map_console.height() as i32 / 2,
        };
        let max_diff = Point {
            x: self.map_console.width() / 2 - CAMERA_STRICTNESS as i32,
            y: self.map_console.height() / 2 - CAMERA_STRICTNESS as i32,
        };
        let diff = self.cursor - camera_center;
        if diff.x.abs() > max_diff.x {
            if diff.x > 0 {
                self.camera.x = min(self.map.width as i32 - self.map_console.width() as i32,
                                    max(0, self.camera.x + (diff.x - max_diff.x)));
            }
            else {
                self.camera.x = min(self.map.width as i32 - self.map_console.width() as i32,
                                    max(0, self.camera.x - (diff.x.abs() - max_diff.x)));
            }
        }
        if diff.y.abs() > max_diff.y {
            if diff.y > 0 {
                self.camera.y = min(self.map.height as i32 - self.map_console.height() as i32,
                                    max(0, self.camera.y + (diff.y - max_diff.y)));
            }
            else {
                self.camera.y = min(self.map.height as i32 - self.map_console.height() as i32,
                                    max(0, self.camera.y - (diff.y.abs() - max_diff.y)));
            }
        }

    }

    pub fn get_zoomed_out_cursor(&self) -> Point<i32> {
        Point {
            x: ((self.cursor.x as f32 / self.map.width as f32) * (self.zoomed_map_console.width() as f32)) as i32 ,
            y: ((self.cursor.y as f32 / self.map.height as f32) * (self.zoomed_map_console.height() as f32)) as i32,
        }
    }
}
