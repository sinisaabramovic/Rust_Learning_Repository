
#[derive(Debug)]
#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64
}

use std::ops::Add;
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

use std::ops::Sub;
impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}

use std::ops::Mul;
impl Mul for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {x: self.x * other.x, y: self.y * other.y}
    }
}

use std::ops::Div;
impl Div for Point {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point {x: self.x / other.x, y: self.y / other.y}
    }
}

pub fn run() {

    let p1 = Point{x: 5.42, y: 3.0};
    let p2 = Point{x: 1.45, y: 42.0};

    let result_add = p1 + p2;
    println!("{:?}", result_add);

    let result_minus = p1 - p2;
    println!("{:?}", result_minus);

    let result_div = p1 / p2;
    println!("{:?}", result_div);

    let result_mul = p1 * p2;
    println!("{:?}", result_mul);
}