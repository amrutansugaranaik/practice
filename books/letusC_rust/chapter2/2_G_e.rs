/*
 * Input three sides, check if it can form a triangle
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

fn main () {
    let side1:i32;
    let side2:i32;
    let side3:i32;

    side1 = get_number ("enter first side : ".to_string());
    side2 = get_number ("enter second side : ".to_string());
    side3 = get_number ("enter third side : ".to_string());

    let mut large:i32 = side1;
    if large < side2 {
        large = side2;
    }
    if large < side3 {
        large = side3;
    }

    // sum of least two sides
    let sum:i32 = side1 + side2 + side3 - large;

    if sum > large {
        println! ("Valid triangle");
    } else {
        println! ("Not valid triangle");
    }
}