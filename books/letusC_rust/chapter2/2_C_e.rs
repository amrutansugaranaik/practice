/*
 * A five digit number is input. Check if it is a pallindrome.
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

fn reverse(mut num:i32) -> i32 {
    let mut rev:i32 = 0;
    while num > 0 {
        rev = rev * 10 + (num % 10);
        num = num / 10;
    }
    return rev;
}

fn main() {
    let input:i32 = get_number ("Enter a five digit number ".to_string());
    if input == reverse(input) {
        println! ("True ");
    } else {
        println! ("False ");
    }
}