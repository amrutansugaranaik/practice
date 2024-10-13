/*
 * A number is input, print its octal form
 */

use std::io;
use std::io::Write;

fn get_num () -> i32 {
    let mut input:String = String::new();
    let num:i32;

    io::stdout().flush().expect ("not able to read");
    io::stdin().read_line(&mut input).expect("Not an integer");
    num = input.trim().parse().unwrap();
    input.clear();
    return num;
}

fn main() {
    let mut num:i32 = get_num();
    let mut ans:i32 = 0;
    let mut initial_zeros:i32 = 0;
    let mut rev:i32 = 0;

    while num > 0 {
        ans = ans * 10 + (num % 8);
        num = num / 8;
        if ans == 0 {
            initial_zeros += 1;
        }
    }

    while ans != 0 {
        rev = rev * 10 + (ans % 10);
        ans = ans / 10;
    }

    print! ("Octal form = {}", rev);
    for _i in 0..initial_zeros {
        print! ("0");
    }
    println! ("");
}