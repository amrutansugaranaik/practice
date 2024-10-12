/*
 * Input two numbers, swap them.
 */

use std::io;
use std::io::Write;

fn main() {
    let mut first:i32;
    let mut second:i32;
    let temp:i32;

    let mut input:String = String::new();

    print! ("Enter first number : ");
    io::stdout().flush().unwrap(); 
    io::stdin()
            .read_line (&mut input)
            .expect ("Failed to read first number");
    first = input
                .trim()
                .parse()
                .expect("Input is not an integere");
    input.clear();

    print! ("Enter second number : ");
    io::stdout().flush().unwrap(); 
    io::stdin()
            .read_line (&mut input)
            .expect ("Failed to read first number");
    second = input
                .trim()
                .parse()
                .expect("Input is not an integere");
    input.clear();

    temp = first;
    first = second;
    second = temp;

    println! ("first = {} second = {}", first, second);
}