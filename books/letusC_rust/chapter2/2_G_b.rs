/*
 * Check if input character is capital, small, number or special character using ASCII value.
 */

use std::io;
use std::io::Write;

fn get_number(s:String) -> char {
    let mut input:String = String::new();
    let ch:char;

    print! ("{}", s);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect ("not able to read from keyboard");
    ch = input.trim().parse().expect("not a character");
    return ch;
}

fn main() {
    let ch:char;

    ch = get_number("Enter character : ".to_string());

    if ch > 'A' && ch < 'Z' {
    println! ("Capital");
    } else if ch > 'a' && ch < 'z' {
    println! ("Small");
    } else if ch > '0' && ch < '9' {
    println! ("Numeric");
    } else {
    println! ("Special character");
    }
}