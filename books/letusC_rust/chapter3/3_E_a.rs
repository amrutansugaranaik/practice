/*
 * Print all prime numbers from 1 to 500.
 * Use nested loop, break and continue.
 */

fn main () {
    for i in 2..=500 {
        let num = i as u32;
        let mut is_prime:bool = true;
        for j in 2..i {
            if num % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            println! ("{} ", num);
        }
    }
}