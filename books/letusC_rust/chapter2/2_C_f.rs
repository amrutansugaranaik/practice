/*
 * If age of three people input through keyboard, find the youngest one.
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
    let first:i32;
    let second:i32;
    let third:i32;

    first = get_number ("Age of first person : ".to_string());
    second = get_number ("Age of second person : ".to_string());
    third = get_number ("Age of third person : ".to_string());

    if first < second && first < third {
        println! ("First person is youngest ");
    } else if second < first && second < third {
        println! ("Second person is youngest ");
    } else if third < second && third < first {
        println! ("Third person is youngest ");
    } else {
        println! ("At least two of them are of same youngest age ");
    }
}