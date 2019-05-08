#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

fn use_slices(slice: &mut[i32]) {
    println!("First element = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

pub fn run() {

    let mut data = [1,2,3,4,5];

    use_slices(&mut data[1..4]);
    //use_slices(&mut data);
    println!("{:?}", data);
}