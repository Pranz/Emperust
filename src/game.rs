
use tcod::console::Offscreen;

use settings::Settings;
use input::UserCommand;
use point::Point;
use map::{Map, get_height_map};

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
            map_console: Offscreen::new(settings.map_window_width as i32,
                                        settings.map_window_height as i32),
            settings: settings,
        }
    }

    pub fn execute_command(&mut self, cmd: UserCommand) {

        use std::cmp::{min, max};
        
        match cmd {
            UserCommand::Move(dir) => {
                let Point {x: dx, y: dy} = dir.to_point();
                
                self.cursor.x = max(0, min(self.map.width as i32,
                                           self.cursor.x + dx));
                self.cursor.y = max(0, min(self.map.height as i32,
                                           self.cursor.y + dy));
            },
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
    
}
