struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn print(&self) {
         println!("My Line is at ({}, {}) and ({}, {})", self.start.x, self.start.y, self.end.x, self.end.y);
    }
}

pub fn run() {

    let my_first_point = Point { x: 3.0, y: 4.0};
    println!("First Point is at ({}, {})", my_first_point.x, my_first_point.y);
    let my_second_point = Point{ x: 1.0, y: 3.0};
    println!("Second Point is at ({}, {})", my_second_point.x, my_second_point.y);

    let my_line = Line{start: my_first_point, end: my_second_point};
    //println!("My Line is at ({}, {}) and ({}, {})", my_line.start.x, my_line.start.y, my_line.end.x, my_line.end.y);
    my_line.print();
}