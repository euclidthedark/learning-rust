fn main () {
    let mut count = 0;

    loop {
        count += 1;
        println!("The count is {}", count);

        if count == 10 { break; }
    }
}
