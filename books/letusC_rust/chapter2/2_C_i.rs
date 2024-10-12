/*
 * length and breadth of a rectangle is input, check if area is more or perimeter.
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
    let length:i32;
    let breadth:i32;
    let area:i32;
    let perimeter:i32;

    length = get_number ("enter length : ".to_string());
    breadth = get_number ("enter breadth : ".to_string());
    area = length * breadth;
    perimeter = 2 * (length + breadth);

    if area - perimeter < 0 {
        println! ("Perimeter is more ");
    } else {
        println! ("Area is more ");
    }
}