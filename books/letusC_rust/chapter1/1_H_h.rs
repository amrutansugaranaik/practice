/*
 * Input a 5 digit number, reverse it.
 */

use std::io;
use std::io::Write;

fn main() {
    let mut num:i32;
    let mut rev:i32 = 0;
    let mut input:String = String::new();

    print! ("Enter a five digit number : ");

    io::stdout().flush().unwrap();
    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from keyboard");
    num = input
            .trim()
            .parse()
            .expect("Not an integer");
    input.clear();

    while num > 0 {
        rev = (rev * 10) + (num % 10);
        num = num / 10;
    }
    println! ("Reverse number = {}", rev);
}