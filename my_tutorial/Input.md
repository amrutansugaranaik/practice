
fn input_i32 () -> i32 {
    let mut input:String = String::new();
    let num:i32;
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    num = input.trim().parse().expect("Not a valid input");
    return num;
}

fn input_i32_line() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

use std::io;

fn input_i64 () -> i64 {
    let mut input:String = String::new();
    let num:i64;
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    num = input.trim().parse().expect("Not a valid input");
    return num;
}

fn input_i64_line() -> Vec<i64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}