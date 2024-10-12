/*
 * Input is a five digit number. Add 1 to each digit.
 * input = 11111 -> output = 22222
 * input = 12391 -> output = 23502
 * input = 99999 -> output = 100000
 */

// NOT WORKING FOR ALL INPUTS. NEED TO CHECK

use std::io;

fn reverse (mut num:i32) -> i32 {
    let mut rev:i32 = 0;
    while num > 0 {
        rev = rev * 10 + (num % 10);
        num = num / 10;
    }
    return rev;
}

fn main () {
    let mut num:i32;
    let mut ans:i32 = 0;
    let mut temp:i32;
    let mut carry:i32 = 0;
    let mut input:String = String::new();

    io::stdin()
            .read_line (&mut input)
            .expect ("failed to read ");
    num = input
            .trim()
            .parse()
            .expect("not a number");
    input.clear();

    while num > 0 {
        temp = num % 10;
        num = num / 10;
        temp = temp + carry + 1;
        carry = 0;
        if temp >= 10 {
            temp = temp % 10;
            temp = temp / 10;
            carry = 1;
        }
        println! ("temp = {} carry = {}", temp, carry);
        ans = ans * 10 + temp;
    }
    println! ("Final answer = {} ", reverse(ans));
}