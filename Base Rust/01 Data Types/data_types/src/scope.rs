use std::mem;

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
}