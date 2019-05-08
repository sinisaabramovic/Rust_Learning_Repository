struct Point<T> {
    x: T,
    y: T
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>
}

struct BaseGeneric<T, V> {
    x: T,
    y: V
}

pub fn run() {

    let a:Point<i32> = Point {x: 10, y: 20}; // We can do explicit
    let a2:Point<i32> = Point {x: -10, y: 30}; // We can do explicit
    let b = Point {x: 1.2, y: 3.4};

    let base = BaseGeneric {x: "Hello", y: 1.2};

    println!("{}", b.x);
    println!("BaseGeneric = {}", base.x);

    let my_line = Line {start: a, end: a2};

    println!("{}", my_line.start.x);
}