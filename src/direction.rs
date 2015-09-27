
use point::Point;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn to_point(&self) -> Point<i32> {
        match *self {
            Direction::Right => Point::new(1, 0),
            Direction::Left  => Point::new(-1, 0),
            Direction::Up    => Point::new(0, -1),
            Direction::Down  => Point::new(0, 1),
        }
    }
}
