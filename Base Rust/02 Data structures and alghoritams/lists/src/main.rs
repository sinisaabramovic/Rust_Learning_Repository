
mod list;
mod foo_model;

use list::List;
use foo_model::FooModel;

fn main() {
    println!("Hello, world!");

    let mut list: List<FooModel> = List::new();

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

    list.print_list();

    println!("Element is at index {:?}", list.get_index(my_foo_1.clone()));

    let my_foo_x = list.first();
    println!("First element in List  my_foo_x is  {:?}", my_foo_x);

    list.remove(my_foo_1);
    list.print_list();

}