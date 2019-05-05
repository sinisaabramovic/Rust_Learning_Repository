#![allow(dead_code)]
use std::mem;

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}

pub fn run() {
    // Stack allocated variable
    let _stack_point: Point = origin();

    // Heap allocated variables
    let _heap_point_1 = Box::new(origin());
    let _heap_point_2 = Box::new(Point{x: 10.0, y: 10.0});
    let mut _mut_heap_point_1 = Box::new(Point{x: 1.0, y: 1.0});

    println!("Stack: _stack_point take up {} bytes", mem::size_of_val(&_stack_point));
    println!("Heap: _heap_point take up {} bytes", mem::size_of_val(&_heap_point_1));

    println!("Values of _heap_point_2 x={} and y={}", _heap_point_2.x, _heap_point_2.y);
    let _another_heap = *_heap_point_2;
    println!("Values of _another_heap x={} and y={}", _another_heap.x, _another_heap.y);

    println!("Initial Values of _mut_heap_point_1 x={} and y={}", _mut_heap_point_1.x, _mut_heap_point_1.y);
    let mut _mut_heap_point_2 = *_mut_heap_point_1;
    
    println!("Initial Values of _mut_heap_point_2 x={} and y={}", _mut_heap_point_2.x, _mut_heap_point_2.y);
    _mut_heap_point_2.x =  _mut_heap_point_2.x + 1.0;
    _mut_heap_point_2.y =  _mut_heap_point_2.y + 1.0;

    // this can not be called in this kind of expressions because _mut_heap_point_1 dereference to _mut_heap_point_2
    // which is mutable variable and in that case we can not guarantee safety of the variable _mut_heap_point_1
    // println!("Initial Values of _mut_heap_point_1 x={} and y={}", _mut_heap_point_1.x, _mut_heap_point_1.y);
    println!("Initial Values of _mut_heap_point_2 x={} and y={}", _mut_heap_point_2.x, _mut_heap_point_2.y);
}