
use tcod::console::{Console, Offscreen};

use settings::Settings;
use input::UserCommand;
use point::Point;
use map::{Map, get_height_map};
use direction::Direction;

use std::cmp::{min, max};

const CAMERA_STRICTNESS : usize = 8;

pub struct Game {
    pub map: Map,
    pub cursor: Point<i32>,
    pub camera: Point<i32>,
    pub settings: Settings,
    pub map_console: Offscreen,
}


impl Game {
    pub fn new(settings: Settings) -> Game {
        let height_map = get_height_map(&settings);
        Game {
            map: Map::new(settings.map_width as usize,
                          settings.map_height as usize,
                          &*height_map),
            cursor: Point::new(5, 5),
            camera: Point::new(0, 0),
            map_console: Offscreen::new(settings.map_window_width as i32,
                                        settings.map_window_height as i32),
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
            UserCommand::RegenMap => {self.regenerate_map(); }
            _ => {},
        }
    }

    pub fn regenerate_map(&mut self) {
        let height_map = get_height_map(&self.settings);
        self.map = Map::new(self.settings.map_width as usize,
                            self.settings.map_width as usize,
                            &*height_map);
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
}
