use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    pub x: u64,
    pub y: u64
}

#[allow(non_snake_case)]
pub fn Point(x: u64, y: u64) -> Point {
    Point {
        x: x,
        y: y,
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rect {
    pub top_left: Point,
    pub bot_right: Point,
}

impl Rect {
    pub fn contains(&self, point: Point) -> bool {
        self.top_left <= point && point <= self.bot_right
    }
}

#[allow(non_snake_case)]
pub fn Rect(x0: u64, y0: u64, x1: u64, y1: u64) -> Rect {
    Rect {
        top_left: Point(x0, y0),
        bot_right: Point(x1, y1)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point(self.x - rhs.x, self.y - rhs.y)
    }
}
