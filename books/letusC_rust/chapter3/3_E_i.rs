/*
 * Enter a number from keyboard and print its multiplication table.
 */

fn main() {
    let mut input:String = String::new();
    let num:i32;

    std::io::stdin().read_line (&mut input).expect("Cannot read from keyboard");
    num = input.trim().parse().expect ("not integer");

    for i in 1..=10 {
        println! ("{} * {} = {} ", num, i, num * i);
    }
}