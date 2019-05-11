
mod list;
mod foo_model;

use list::List;
use foo_model::FooModel;

fn main() {
    println!("Hello, world!");

    let mut list: List<FooModel> = List::new();
    
    let my_foo_x = list.first();
    match my_foo_x {
        Some(value) => println!("First element in List  my_foo_x is  {:?}", value),
        None => println!("Empty array!")
    }

    let mut big_list: List<FooModel> = List::new();

    let mut list_2: List<u32> = List::new();

    // for i in 0 .. <usize>::max_value() {
    //     println!("Curren : {}", i);
    //     big_list.add(FooModel {name: "William".to_string(), id_num: i}); 
    // }
    // println!("Big list created");

    big_list.print_list();

    let my_foo_not = FooModel {name: "John Snow".to_string(), id_num: 0};

    let my_foo_1 = FooModel {name: "John".to_string(), id_num: 10};
    let my_foo_2 = FooModel {name: "Bill".to_string(), id_num: 20};
    let mut my_foo_3 = FooModel {name: "Gunter".to_string(), id_num: 30};
    let mut my_foo_4 = FooModel {name: "Steve".to_string(), id_num: 40};
    let my_foo_5 = FooModel {name: "Arnold".to_string(), id_num: 50};

    list.add(my_foo_1.clone());
    list.add(my_foo_2.clone());
    list.add(my_foo_3.clone());   
    list.add(my_foo_4.clone()); 
    list.add(my_foo_5.clone());  
    list.add(FooModel {name: "William".to_string(), id_num: 60}); 

    list_2.add(1);
    list_2.add(2);
    list_2.add(3);
    list_2.add(4);

    list.print_list();
    list_2.print_list();
    println!("Element is at index {:?}", list_2.get_index(1));

    println!("Non inserted Element is at index {:?}", list.get_index(my_foo_not.clone()));

    println!("Element is at index {:?}", list.get_index(my_foo_1.clone()));

    let my_foo_x = list.first();
    match my_foo_x {
        Some(value) => println!("First element in List  my_foo_x is  {:?}", value),
        None => println!("Empty array!")
    }
    //println!("First element in List  my_foo_x is  {:?}", my_foo_x.unwrap());

    list.remove_element(my_foo_1);
    list.print_list();

}