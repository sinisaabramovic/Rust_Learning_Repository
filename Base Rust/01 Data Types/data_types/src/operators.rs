use std::mem;

pub fn run() {
    // Arithimitc
    println!("Basic arithmetic operations");
    let mut _number_1 = 2 + 3 * 4;
    println!("_number_1 = {}", _number_1);
    _number_1 = _number_1 + 1;
    println!("expression (_number_1 = _number_1 + 1) _number_1 = {}", _number_1);
    let _number_2 = _number_1 / 2;
    println!("expression (_number_1 = {} / 2) _number_1 = {}", _number_1, _number_2);

    println!("Mod");
    let _number_3 = _number_2 % 3;
    println!("expression ({} % 3) _number_1 = {}", _number_2, _number_3);

    println!("Using mathematical functions");
    let _power_of :u32 = 2;
    let _a_cubed_value = i32::pow(_number_1, _power_of);
    println!("expression (i32::pow({}, {})) result = {}", _number_1, _power_of,_a_cubed_value);

    let _small_float :i32 = 3;
    let _float :f64 = 34.43;
    let _power_of_pow = f64::powi(_float, _small_float);
    println!("expression (i32::pow({}, {})) result = {}", _float, _small_float,_power_of_pow);

    // Bitwise operators only available for integers
    println!("Bitwise operators");
    let _bit_num_1 = 1;
    let _bit_num_2 = 2;
    let _or_bitwise = _bit_num_1 | _bit_num_2; // | OR, & AND, ^ XOR, ! NOR
    println!("expression: _small_bitwise = {:b} | {:b} = {:b}",_bit_num_1, _bit_num_2, _or_bitwise);
    let _and_bitwise = _bit_num_1 & _bit_num_2; // | OR, & AND, ^ XOR, ! NOR
    println!("expression: _small_bitwise = {:b} & {:b} = {:b}",_bit_num_1, _bit_num_2, _and_bitwise);
    let _xor_bitwise = _bit_num_1 ^ _bit_num_2; // | OR, & AND, ^ XOR, ! NOR
    println!("expression: _small_bitwise = {:b} ^ {:b} = {:b}",_bit_num_1, _bit_num_2, _xor_bitwise);
    let _nor_bitwise = ! _bit_num_1; // | OR, & AND, ^ XOR, ! NOR
    println!("expression: _small_bitwise = ! {:b} = {:b}",_bit_num_1, _nor_bitwise);

    println!("Bitwise shifting");
    let _number_to_shift = 1;
    let _places_to_shift = 10;
    let _shift_result = _number_to_shift << _places_to_shift; // Shift 10 places in 1

    println!("Binary:");
    println!("expression:  {:b} << {:b} = {:b}",_number_to_shift, _places_to_shift, _shift_result);
    println!("Decimal:");
    println!("expression:  {} << {} = {}",_number_to_shift, _places_to_shift, _shift_result);

    println!("Logical operators");
    let _logical_result = std::f64::consts::PI < 4.0;
    
}