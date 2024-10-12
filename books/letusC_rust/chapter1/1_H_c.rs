/*
 * Marks of a student in 5 subjects are input, find aggregate marks and percentage.
 * Max mark in each subject is 100.
 */

use std::io;
use std::io::Write;

fn main () {
    
    let mut sum:i32 = 0;
    let percentage:f32;    // will be updated once

    println! ("Enter the marks in 5 subject : ");

    for i in 0..5 {
        print! ("Enter mark for subject {} : ", i);

        let mut input:String = String::new();
        io::stdout().flush().unwrap();  // flush the previous \t \n etc
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input from keyboard ");
        io::stdout().flush().unwrap();  // This is not required, just to be safe

        let temp:i32 = input
                        .trim()
                        .parse()
                        .expect("Input must be integer ");
        sum = sum + temp;
    }
    percentage = (sum as f32) * 100.0 / 500.0;

    println! ("Aggregate = {} percentage = {}%", sum, percentage);
}