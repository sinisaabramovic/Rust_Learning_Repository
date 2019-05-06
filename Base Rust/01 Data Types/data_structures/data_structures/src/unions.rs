#![allow(dead_code)]
#![allow(unused_variables)]

union int_or_float_u {
    i: i32,
    f :f32
}

fn process_int_or_float(int_or_float: int_or_float_u) {
    unsafe {
        match int_or_float {
            int_or_float_u{ i: 43 } => { println!("The meaning of life"); }
            int_or_float_u{ f } => { println!("The float value = {}", f); }
            int_or_float_u{ i } => { println!("The int value = {}", i); }
        }
    }
}

pub fn run() {

    let mut int_or_float = int_or_float_u{ i: 256 };

    // modifaying unions are unsafe operations
    unsafe { int_or_float.i = 42; }

    unsafe { 
        println!("Value of int_or_float = {}", int_or_float.i);
    }

    process_int_or_float(int_or_float);
    process_int_or_float(int_or_float_u{ i: 128 });
    
}