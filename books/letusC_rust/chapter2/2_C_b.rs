/*
 * Check if input number is even or odd.
 */

use std::io;
use std::io::Write;

fn get_number(s:String) -> i32 {
    let mut input:String = String::new();
    let num:i32;

    print! ("{}", s);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect ("not able to read from keyboard");
    num = input.trim().parse().expect("not a floating point number");
    return num;
}

fn main() {
    let num:i32 = get_number("Enter number : ".to_string());
    if (num & 1) == 1 {
        println! ("{} is Odd", num);
    } else {
        println! ("{} is Even", num);
    }
}