fn main () {
    let mut n = 0;

    loop {
        n += 1;
        if n == 10 { break; }

        match n {
            1 => println!("The first matcher."),
            2 => println!("The second matcher."),
            _ => println!("It is the thrid matcher."),
        }
    }
}
