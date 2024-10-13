/*
 * input two number a and b and find a^b
 */

use std::io;
use std::io::Write;

fn main() {
    let mut input:String = String::new();
    let a:i32;
    let b:i32;

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect ("Not able to read from keyboard");
    a = input.trim().parse().expect ("Not integer");
    input.clear();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect ("Not able to read from keyboard");
    b = input.trim().parse().expect ("Not integer");
    input.clear();

    let mut c:i32 = 1;

    for _i in 0..b {
        c = c * a;
    }

    println! ("{}^{} = {}", a, b, c);
}