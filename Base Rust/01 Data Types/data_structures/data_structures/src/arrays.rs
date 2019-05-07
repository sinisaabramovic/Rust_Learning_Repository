#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

pub fn run() {

    // 5 integers
    let mut array: [i32;5] = [1,2,3,4,5];
    println!("array has {} elements, first is {}", array.len(),array[0]);
    array[0] = 321;
    println!("Array[0] = {}", array[0]);

    println!("{:?}", array);

    // can not compare arays with different size!
    if array == [321,2,3,4,5] {
        println!("match!");
    }

    // 10 elements equal to 1
    let array_b = [1; 10];

    for i in 0..array_b.len() {
        println!("{}", array_b[i]);
    }

    println!("array_b took up {} bytes", mem::size_of_val(&array_b));

    // Multi dimensional array (example 2D)
    let matrix:[[f32;3]; 3] = 
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
        [0.0, 0.0, 3.0]
    ];

    println!("{:?}", matrix);

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            // diagonal print
            if i == j {
                println!("Matrix[{}][{}] = {}", i, j, matrix[i][j]);
            }
        }
    }

}