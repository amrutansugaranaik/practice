/*
 * Library fine for delayed book return 
 *      : 0-5 days - 50 paise
 *      : 6-10 days - 1 rupees
 *      : 11-30 days - 5 rupees
 *      : 30+ days - membership cancellation
 */
 
use std::io;
use std::io::Write;

fn get_number(s:String) -> i32 {
    let mut input:String = String::new();
    let num:i32;

    print! ("{}", s);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect ("not able to read from keyboard");
    num = input.trim().parse().expect("not a floating point number");
    return num;
}

fn main () {
    let days:i32 = get_number ("Enter delay in number of days : ".to_string());
    if days <= 5 {
        println! ("50 paise ");
    } else if days <= 10 {
        println! ("1 rupee ");
    } else if days <= 30 {
        println! ("5 rupees ");
    } else {
        println! ("membership cancelled ");
    }
}