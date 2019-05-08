fn say_hello_function() {
    println!("Hello");
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

pub fn run() {

    let say_hello = say_hello_function;
    say_hello();

    // Closure or lambda
    let plus_one = |x:i32| -> i32 { x + 1 };

    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let plus_two = |x| {
        let mut z = x;
        z += 2;
        z
    };

    println!("{} + 2 = {}", a, plus_two(a));

    // T: by value
    // T& by ref
    // &mut & mutable referece

    let plus_three = |x:&mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);

    // High order functions

    let limit = 500;
    let mut sum = 0;

    for i in 0.. {
        let isq = i * i;

        if isq > limit { break; }
        else if is_even(isq) { sum += isq; }
    }

    println!("loop sum = {}", sum);

    let sum2 = 
        (0..).map(|x| x*x)
        .take_while(|&x| x <= limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
        
    println!("loop sum = {}", sum2);
}