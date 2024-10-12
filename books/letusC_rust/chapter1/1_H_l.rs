/*
 * Selling price and profit of 15 items are input. find the cost price of each item.
 */

use std::io;
use std::io::Write;

const TOTAL:i32 = 15;

fn read_float () -> f32 {
    let mut input:String = String::new();
    let num:f32;
    io::stdout().flush().unwrap();
    io::stdin()
            .read_line(&mut input)
            .expect ("Failed to read from keyboard");
    num = input
            .trim()
            .parse()
            .expect ("not float");
    input.clear();
    return num;
}

fn main () {
    let selling_price:f32;
    let profit:f32;

    print! ("Selling price : ");
    selling_price = read_float();
    print! ("profit : ");
    profit = read_float();

    println! ("cost price per item = {} ", (selling_price - profit) / (TOTAL as f32));
}