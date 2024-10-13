/*
 * Take away game with start value 21, value to be removed are 1,2,3 or 4 and
 * the computer starts and should win.
 */

//use std::io::{Self, Write};
use std::io;
use std::io::Write;

fn main() {
    let mut input:String = String::new();
    let mut total:i32 = 21;  // start number
    let mut user_input:i32;

    println! ("Computer starts ... ");
    total -= 1; // computer takes 1 to make the number multiple of 5
    println! ("Computer takes 1. Remaining = {}", total);

    loop {
        print! ("Your turn. Select between 1, 2, 3 or 4.. : ");

        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).expect ("Failed to read from keyboard");
        user_input = input.trim().parse().expect ("Expected an integer");

        total = total - user_input;
        println! ("User selected : {}, total remains = {}", user_input, total);

        if total == 0 {
            println! ("You loose ");
            break;
        }

        // if total is less than 5, computer takes such that use has only 1 left and is forced to take.
        if total < 5 {
            println! ("Computer takes {} , total remains = {}", total - 1, 1);
            total = 1;
        } else {
            // COmputer needs to take a number which makes the total a multiple of 5.
            total = total - (5 - user_input);
            println! ("Computer takes {} , total remains = {}", 5 - user_input, total);
        }
    }
}