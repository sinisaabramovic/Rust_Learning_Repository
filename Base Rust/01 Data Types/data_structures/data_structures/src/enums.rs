#![allow(dead_code)]
#![allow(unused_variables)]

enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8), // by tuple
    CMYK{cyan: u8, magenta: u8, yellow: u8, black: u8} // by struct
}

pub fn run() {

    let color: Color = Color::Red;
    let black_color = Color::RGBColor(0,0,0);
    let rgb_color = Color::RGBColor(255,128,64);
    let cmyk_color = Color::CMYK{cyan: 0, magenta: 128, yellow: 64, black: 255};

    let heap_enum = Box::new(Color::Blue);

     match *heap_enum {
        Color::Red => println!("The Red color"),
        Color::Green => println!("The Green color"),
        Color::Blue => println!("The Blue color"),
        Color::RGBColor(0,0,0) => println!("Black color"),
        Color::RGBColor(r,g,b) => println!("RGB {} {} {}", r, g, b),
        Color::CMYK{cyan:_, magenta:_, yellow:_, black: 255} => println!("Black by Cmyk"),
        _ => () // Do anything
    }

    match color {
        Color::Red => println!("The Red color"),
        Color::Green => println!("The Green color"),
        Color::Blue => println!("The Blue color"),
        Color::RGBColor(0,0,0) => println!("Black color"),
        Color::RGBColor(r,g,b) => println!("RGB {} {} {}", r, g, b),
        Color::CMYK{cyan:_, magenta:_, yellow:_, black: 255} => println!("Black by Cmyk"),
        _ => () // Do anything
    }

    match black_color {
        Color::Red => println!("The Red color"),
        Color::Green => println!("The Green color"),
        Color::Blue => println!("The Blue color"),
        Color::RGBColor(0,0,0) => println!("Black color"),
        Color::RGBColor(r,g,b) => println!("RGB {} {} {}", r, g, b),
        Color::CMYK{cyan:_, magenta:_, yellow:_, black: 255} => println!("Black by Cmyk"),
        _ => () // Do anything
    }

     match rgb_color {
        Color::Red => println!("The Red color"),
        Color::Green => println!("The Green color"),
        Color::Blue => println!("The Blue color"),
        Color::RGBColor(0,0,0) => println!("Black color"),
        Color::RGBColor(r,g,b) => println!("RGB {} {} {}", r, g, b),
        Color::CMYK{cyan:_, magenta:_, yellow:_, black: 255} => println!("Black by Cmyk"),
        _ => () // Do anything
    }

     match cmyk_color {
        Color::Red => println!("The Red color"),
        Color::Green => println!("The Green color"),
        Color::Blue => println!("The Blue color"),
        Color::RGBColor(0,0,0) => println!("Black color"),
        Color::RGBColor(r,g,b) => println!("RGB {} {} {}", r, g, b),
        Color::CMYK{cyan:_, magenta:_, yellow:_, black: 255} => println!("Black by Cmyk"),
        _ => () // Do anything
    }
}