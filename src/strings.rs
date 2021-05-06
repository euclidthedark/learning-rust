fn main () {
    /*
     * There are two string types in Rust:
     * 1) String
     * 2) String slice 
     * */

    // A string is a collection of chars.
    
    let full_name = String::from("Waylon Duff");
    println!("My Name is {}", full_name);

    // A string slice is a pointer to a section of a slice

    let first_name = &full_name[0..6];

    println!("Yes, {} is my first name.", first_name);

    /*
     * There are 4 scaler types:
     * 1) Bool
     * 2) Ints - Can be signed or unsigned
     * 3) Chars
     * 4) And double floats
     * */

    let is_a_bool = true;

    let an_integer: i8 = 10;

    let a_char = 'a';

    let a_float: f32 = 4.5;

    format!("{}, {}, {}, {}", is_a_bool, an_integer, a_char, a_float);
}
