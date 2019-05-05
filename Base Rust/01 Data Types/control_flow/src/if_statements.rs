
pub fn run() {
    
    let temperature = 15;

    if temperature > 30 {
        println!("Outside is a really hot");
    } else if temperature < 10 {
        println!("Outside is a really cold");
    } else {
        println!("Temperature is OK :)");
    }

    let day = if temperature > 20 {"sunny"} else {"cloudy"};
    println!("Today is {} day", day);
}