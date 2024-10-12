/*
 * Check if input number is leap year or not.
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
    let year:i32 = get_number ("Enter year : ".to_string());

    if year % 400 == 0 {
        println! ("{} is Leap year", year);
    } else if (year % 4 == 0) && (year % 100 != 0) {
        println! ("{} is Leap year", year);
    } else {
        println! ("{} is not a Leap year", year);
    }
}