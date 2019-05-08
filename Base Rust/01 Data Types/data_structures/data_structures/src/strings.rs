#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

pub fn run() {
    let simple_string: &'static str = "hello there!"; // &str = string slice

    for i in simple_string.chars() {
        print!("{}", i);
    }

    println!("");

    if let Some(first_character) = simple_string.chars().nth(0) {
        println!("First letter is {} ", first_character);
    }


    // heap allocated construct
    // String

    let mut letters = String::new();
    let mut a = 'a' as u8;

    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    // convrsion &str <> String
    let u:&str = &letters;
    let x_val = 23.45;

    let mut abc = x_val.to_string();
    abc.push_str(" Value is OK added ");

    println!("{}", abc.replace("OK", "Cool"));
    abc.push_str(&letters);
    println!("{}", abc);
    // concatentation
    //let z = letters + &letters;
}