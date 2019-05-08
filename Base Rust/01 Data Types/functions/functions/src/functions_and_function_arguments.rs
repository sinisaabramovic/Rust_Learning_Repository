fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase_by_one(x: &mut i32) {
    *x += 1;
}

fn product_abs(x: i32, y: i32) -> i32 {
   x.abs() * y.abs()
}

fn product(x: i32, y: i32) -> i32 {
    if x == 0 {
        return 0;
    }
   return x * y;
}

struct Point {
    x: f64,
    y: f64
}

impl Point {
    fn len(&self) -> f64 {
        return self.x * self.y;
    }
}

pub fn run() {

    print_value(32);
    let mut z = 1;
    increase_by_one(&mut z);
    print_value(z);

    let a = 3;
    let b = 5;
    let product_value = product(a, b);
    print_value(product_value);
}