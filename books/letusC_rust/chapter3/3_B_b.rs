/*
 * find factorial of a number input.
 */

use std::io;

fn main() {
    let mut input:String = String::new();
    let mut num:i32;

    io::stdin()
            .read_line(&mut input)
            .expect ("Failed to read from keyboard");
    num = input
            .trim()
            .parse()
            .expect ("Not an integer");
    let mut factorial:i64 = 1;
    while num >= 1 {
        factorial = factorial * (num as i64);
        num -= 1;
    }

    println! ("Factorial = {} ", factorial);
}