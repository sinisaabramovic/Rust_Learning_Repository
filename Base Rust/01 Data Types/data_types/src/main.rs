mod primitive_data_types;
mod operators;
mod scope;
mod stack_and_heap;

fn main() {
    // primitive_data_types::run();
    // operators::run();
//   scope::run();

//   println!("SHOW MY MEANING_OF_LIFE = {} OUTSIDE THE MODULE", scope::MEANING_OF_LIFE);
//   println!("SHOW MY SOME_UNMUTE_STATIC = {} OUTSIDE THE MODULE", scope::SOME_UNMUTE_STATIC);
//   unsafe {
//         println!("SHOW MY SOME_STATIC = {} OUTSIDE THE MODULE", scope::SOME_STATIC);
//   }

    stack_and_heap::run();
}
