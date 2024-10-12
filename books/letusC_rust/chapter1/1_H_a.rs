/*
 * Basic salary is input from keyboard. DA=40%, HRA=20%. Print gross salary.
 */

 use std::io;

 fn main () {
    let mut basic:f32 = 0.0;
    let mut gross:f32 = 0.0;

    let mut input:String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    basic = input.trim().parse().expect("Not a valid input");

    gross = basic + (basic * 0.2) + (basic * 0.4);

    println! ("Gross salary = {} ", gross);
 }