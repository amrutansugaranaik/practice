/*
 * cost and selling price is input, print the profit/loss
 */

use std::io;
use std::io::Write;

fn get_number(s:String) -> f32 {
    let mut input:String = String::new();
    let num:f32;

    print! ("{}", s);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect ("not able to read from keyboard");
    num = input.trim().parse().expect("not a floating point number");
    return num;
}

fn main() {
    let sp:f32; // selling price
    let cp:f32; // cost price

    cp = get_number("Cost price : ".to_string());
    sp = get_number("Selling price : ".to_string());

    if sp > cp {
        println! ("Profit = {}", sp - cp);
    } else if sp < cp {
        println! ("Loss = {}, ", cp - sp);
    } else {
        println! ("No profit no loss");
    }
}