pub fn run() {

    // range
    for i in 1..11 {        
        // ok lets try print all the odds 
        if i % 2 == 1 {continue;}
        if i == 8 {break;}
        println!("i = {}", i);
    }

    let from = 10;
    let to = 50;

    for i in (from..to) {
        println!("{}", i);
    }
    
    for (i, j) in (from..to).enumerate() {
        println!("{}: {}", i, j);
    }
}
