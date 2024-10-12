/*
 * return absolute value of the number entered.
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
    let num:i32;

    num = get_number ("enter number : ".to_string());

    if num < 0 {
        println! ("{}", num * -1);
    } else {
        println! ("{}", num);
    }
}