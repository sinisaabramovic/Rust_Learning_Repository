use std::mem;

pub fn run() {
      // number is a name of the variable
    println!("Integers:");
    let _number :u8 = 127; // number is single byte
    println!("_number = {}, size = {} bytes",
            _number,
             mem::size_of_val(&_number));

    let mut _mutable_number :i8 = 127;
    println!("_mutable_number = {}", _mutable_number);

    _mutable_number = 32;
    println!("_mutable_number = {}, size = {} bytes",
     _mutable_number,
      mem::size_of_val(&_mutable_number));

    let mut _some_big_number = 123456789;
    println!("_some_big_number = {}, size = {} bytes",
     _some_big_number,
      mem::size_of_val(&_some_big_number));

      _some_big_number = -123124232;

    println!("_some_big_number = {} after modification, size = {} bytes",
     _some_big_number,
      mem::size_of_val(&_some_big_number));

    // integers numbers can be
    // u8, u8, i16, u16, i32, u32, i64, u64

    // useful when we need to know the size of the variable according to
    // size of the memory adress of the OS on we running our app
    let _integral :isize = 123;
    let _size_of_integral = mem::size_of_val(&_integral);
    println!("_integral = {}, takes up {} bytes, {}-bit OS",
     _integral,
     _size_of_integral, 
     _size_of_integral * 8);

    println!("Characters:");
     let _character = 'x';
     println!("_character = {}, size = {} bytes",
      _character,
       mem::size_of_val(&_character));

    println!("Floating points:");
    let _float_number = 2.5; // double precision value f64
    println!("_float_number = {}, size = {} bytes",
      _float_number,
       mem::size_of_val(&_float_number));

    let _small_float_number :f32= 2.5; // double precision value f64
    println!("_small_float_number = {}, size = {} bytes",
      _small_float_number,
       mem::size_of_val(&_small_float_number));

    println!("Booleans:");
    let _some_bool = false;
     println!("_some_bool = {}, size = {} bytes",
      _some_bool,
       mem::size_of_val(&_some_bool));
}