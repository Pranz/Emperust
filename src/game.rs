
use settings::Settings;
use input::UserCommand;
use point::Point;
use map::Map;

pub struct Game {
    pub map: Map,
    pub cursor: Point<i32>,
}

impl Game {
    pub fn new(settings: &Settings) -> Game {
        Game {
            map: Map::new(settings.map_width as usize,
                          settings.map_height as usize),
            cursor: Point::new(5, 5),
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
            _ => {},
        }
    }
}
