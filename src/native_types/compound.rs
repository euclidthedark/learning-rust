fn main () {
    /*
     * There are two compound types:
     * 1) Tuples
     * 2) Array
     * */

    // Tuples
    
    let a_tuple: (char, u8, String) = ('a', 0, String::from("A thingy!"));
    let (a_char, an_unsigned_int, a_string) = a_tuple;
    let a_list_of_ints = [1, 2, 3, 4];

    println!("{}, {}, {}", a_char, an_unsigned_int, a_string);

    for an_int in a_list_of_ints.iter() {
        println!("{}", an_int);
    }
}
