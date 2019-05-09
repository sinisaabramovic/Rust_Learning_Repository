
use std::mem;

trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

// this is call in runtime
fn print_it_to(z: &Printable) {
    println!("{}", z.format());
}

pub fn run() {

    let a = 123;
    let b = "hello".to_string();

    print_it_to(&a);
    print_it_to(&b);
}