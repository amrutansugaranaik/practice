/*
 * Input distance in kilometer, convert into meter, inch, foot and centimeter
 */

 use std::io;

 fn main() {
    let km:f32;

    let mut input = String::new();
    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input kilometer");

    km = input.trim().parse().expect("invalid input");
    let cm = km * 100000.0;
    let m = km * 1000.0;
    let inch:f32 = cm / 2.54;
    let foot = inch / 12.0;

    println! ("km = {} meter = {} cm = {}", km, m, cm);
    println! ("km = {} inch = {} foot = {}", km, inch, foot);
 }