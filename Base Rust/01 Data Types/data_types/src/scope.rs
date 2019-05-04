use std::mem;

pub const MEANING_OF_LIFE :u8 = 42; // no fixed address
pub static mut SOME_STATIC :u8 = 43;
pub static SOME_UNMUTE_STATIC :u8 = 44;

fn scope_and_shadowing() {
    let _some_variable = 127;

    {
        let _some_scope_varialbe = 256;
        // this variable shadows oustide variable
        // this is dangeruos!
        let _some_variable = 42;
        println!("_some_scope_varialbe = {}", _some_scope_varialbe);
        println!("inside in scope _some_variable = {}", _some_variable);
    }

    println!("outside _some_variable = {}", _some_variable);
}

pub fn run() {
    scope_and_shadowing();

    // OK using unsafe is not the smartest idea
    // but it neccesseary when we wish to use mutable static variables
    unsafe {
        println!("SHOW MY STATIC = {}", SOME_STATIC);
        SOME_STATIC = 56;
    }    

    // if the static variable is non static then we can just use it as is (like a constant, yeahhh)
    println!("SHOW MY SOME_UNMUTE_STATIC = {}", SOME_UNMUTE_STATIC);
}