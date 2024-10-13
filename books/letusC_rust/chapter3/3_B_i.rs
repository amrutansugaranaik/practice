/*
 * in a set of numbers, find the difference between largest and smallest.
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

fn main() {
    let mut total:i32 = get_num();
    let mut input:i32;
    let mut min:i32 = 99999999;   // Some large number
    let mut max:i32 = -99999999;  // Some small number

    print! ("Enter the numbers : ");

    loop {
        input = get_num();
        if input > max {
            max = input;
        }
        if input < min {
            min = input;
        }
        total -= 1;
        if total == 0 {
            break;
        }
    }

    println! ("range = {} ", max - min);
    
}