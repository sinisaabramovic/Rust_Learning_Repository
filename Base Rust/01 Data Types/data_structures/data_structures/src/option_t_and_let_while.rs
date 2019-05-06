#![allow(dead_code)]
#![allow(unused_variables)]

// Option is a type which indicate presence of value

pub fn run() {
    let x = 3.0;
    let y = 2.0;

    // Option can be one of state it can be
    // Some or None
    let result: Option<f64> = if (y != 0.0) {
        Some(x / y)
    } else {
        None
    };

    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("Cannot devide {} by {}", x, y)
    }

    if let Some(z) = result { println!("z = {}", z); }    
}