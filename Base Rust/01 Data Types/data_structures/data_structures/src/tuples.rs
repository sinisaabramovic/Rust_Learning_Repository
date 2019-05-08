#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

fn sum_and_product(x:i32, y:i32) -> (i32, i32) {
    (x+y, x*y)
}

pub fn run()  {

    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring

    let (a, b) = sp;
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, a, b);
}