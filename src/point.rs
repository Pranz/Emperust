
use std::ops::{Add, Sub, Neg};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T>{
        Point {x: x, y: y}
    }
    
    #[inline(always)]
    pub fn map<F: Fn(T) -> T>(self, f: F) -> Point<T> {
        Point {x: f(self.x), y: f(self.y)}
    }
}

impl<T: Add> Add for Point<T> {
    type Output = Point<<T as Add>::Output>;

    #[inline(always)]
    fn add(self, other: Point<T>) -> Point<<T as Add>::Output> {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

impl<T: Neg> Neg for Point<T> {
    type Output = Point<<T as Neg>::Output>;

    #[inline(always)]
    fn neg(self) -> Point<<T as Neg>::Output> {
        Point {x : self.x.neg(), y: self.y.neg()}
    }
}

impl<T : Sub> Sub for Point<T> {
    type Output = Point<<T as Sub>::Output>;

    #[inline(always)]
    fn sub(self, other: Point<T>) -> Point<<T as Sub>::Output> {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}
