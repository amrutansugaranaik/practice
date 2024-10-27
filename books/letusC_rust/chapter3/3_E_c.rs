/*
 * Program to find
 * 1/1! + 2/2! + 3/3! + ... for 7 terms.
 */

fn factorial (n:i32) -> i32 {
    let mut num:i32 = 1;
    for i in 1..=n {
        num *= i;
    }
    return num;
}

fn main () {
    let mut result:f32 = 0.0;
    for i in 1..=7 {
        result = result + (i as f32 / factorial(i) as f32);
    }
    println! ("{}", result);
}