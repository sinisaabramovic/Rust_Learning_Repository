#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

fn how_many(x:i32) -> &'static str {

    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        z @ 9...11 => "lots of",
        _ if (x % 2 == 0) => "some",
        _ => "a few"
    }
}

pub fn run() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x , how_many(x));
    }

    let point = (0, 4);

    match (point) {
        (0,0) => println!("At origin!"),
        (0,y) => println!("On x axis, y = {}", y),
        (x,0) => println!("On y axis, x = {}", x),
        (x,y) => println!("({},{})",x, y)
    }
}