/*
 * User enters n numbers. Print number of positive, negative and zeros
 */

use std::io;
use std::io::Write;

fn get_num () -> i32 {
    let mut input:String = String::new();
    let num:i32;

    io::stdout().flush().expect ("not able to read");
    io::stdin().read_line(&mut input).expect("Not an integer");
    num = input.trim().parse().unwrap();
    input.clear();
    return num;
}

fn main () {
    let count:i32;
    let mut input:i32;
    let mut zero:i32 = 0;
    let mut pos:i32 = 0;
    let mut neg:i32 = 0;

    print! ("Enter number of inputs : ");
    count = get_num();

    for _i in 0..count {
        input = get_num();

        if input > 0 {
            pos += 1;
        } else if input < 0 {
            neg += 1;
        } else {
            zero += 1;
        }
    }

    println! ("pos = {} neg = {} zero = {}", pos, neg, zero);
}