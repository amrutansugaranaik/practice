/*
 * If three angles are input, check if they are valid angles of a triangle
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
 
 fn main() {
     let first:i32;
     let second:i32;
     let third:i32;
 
     first = get_number ("first angle : ".to_string());
     second = get_number ("second angle : ".to_string());
     third = get_number ("third angle : ".to_string());
 
     if first + second + third == 180 {
         println! ("Valid angle ");
     } else {
         println! ("Invalid triangle ");
     }
 }