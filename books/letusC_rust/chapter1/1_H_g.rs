/*
 * Input a five digit number, add all the digits.
 */

use std::io;
use std::io::Write;

fn main() {
    let mut num:i32;
    let mut sum:i32 = 0;
    let mut input:String = String::new();

    print! ("Enter a five digit number : ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    num = input
            .trim()
            .parse()
            .expect("input is not integer");
    
    while num > 0 {
        sum = sum + (num % 10);
        num = num / 10;
    }

    println! ("Sum of the digits = {}", sum);
}