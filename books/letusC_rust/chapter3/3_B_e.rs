/*
 * Print Armstrong numbers from 0 to 500
 * Armstrong number 153 = 1*1*1 + 5*5*5 + 3*3*3
 */

fn cube_sum (mut num:i32) -> i32 {
    let mut sum:i32 = 0;
    let mut temp:i32;
    while num > 0 {
        temp = num % 10;
        sum = sum + (temp * temp * temp);
        num = num / 10;
    }
    return sum;
}

fn main() {
    for i in 0..=500 {
        if i == cube_sum(i) {
            println! ("{} is Armstrong ", i);
        }
    }
}

// I think this definition of Armstrong number is wrong but based on the definition given
// in the problem, the code is written accordingly.