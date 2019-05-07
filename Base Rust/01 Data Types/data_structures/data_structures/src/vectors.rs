#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

pub fn run() {
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    println!("vector = {:?}", vector);

    vector.push(42);
    println!("vector = {:?}", vector);

    let index:usize = 0;
    vector[index] = 123;

    println!("vector[0] = {}", vector[index]);

    // returns option type
    let second_index:usize = 32;
    let mut value:i32;
    match vector.get(second_index) {
        Some(x) => value = *x,
        None => value = 0
    };

    println!("The value = {:?}", value);

    for x in &vector {
        println!("{}", x);
    }

    vector.push(66);

    let last_element = vector.pop(); // Option
    println!("last elemet is {:?}", last_element);

    while let Some(x) = vector.pop() {
        println!("{}", x);
    }

    println!("vector = {:?}", vector);
}