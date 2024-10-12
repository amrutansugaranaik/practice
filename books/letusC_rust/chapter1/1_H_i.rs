/*
 * input a 4 digit number, print the sum of first and last digit
 */

use std::io;

fn main() {
    let mut num:i32;
    let mut sum:i32 = 0;
    let mut input:String = String::new();

    io::stdin()
            .read_line(&mut input)
            .expect("not able to read");
    
    num = input
            .trim()
            .parse()
            .expect("not an integer");
    
    sum = sum + (num % 10);

    while num > 10 {
        num = num / 10;
    }

    sum = sum + num;

    println! ("sum of first and last digit = {}", sum);
}