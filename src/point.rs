

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Point<T> {
    pub fn new(x: T, y: T){
        Point {x: x, y: y}
    }
}

impl<T: Add> Add for Point<T> {
    type Output = Vector<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}
