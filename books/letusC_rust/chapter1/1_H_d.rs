/*
 * Input temperature in farhenheit. Convert to centegrade.
 */

 use std::io;

 fn main() {
    let f:f32;
    let c:f32;

    let mut input:String = String::new();

    io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
    f = input
            .trim()
            .parse()
            .expect ("Input is not a float number");
    c = (f - 32.0) / 9.0 * 5.0;

    println! ("F = {}  C = {}", f, c);
 }