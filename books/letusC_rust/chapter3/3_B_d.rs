/*
 * Print ASCII values of numbers from 0 to 255.
 */

fn main() {
    for i in 0..255 {
        println! ("ASCII value of {} is {}", (i as u8 as char), i);
    }
}