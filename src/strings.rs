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
}
