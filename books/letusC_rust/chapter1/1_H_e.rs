/*
 * Input length and breadth of rectangle and radius of a circle from keyboard.
 * Output the area and perimeter of the rectangle and area and circumference of the circle.
 */

use std::io;
use std::io::Write;

fn main() {
    let length:i32;
    let breadth:i32;
    let radius:i32;

    let mut input:String = String::new();

    print! ("Enter length : ");
    io::stdout().flush().unwrap(); 
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    
    length = input
                .trim()
                .parse()
                .expect ("length is not an integer");
    input.clear();

    print! ("Enter bredth : ");
    io::stdout().flush().unwrap(); 
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    
    breadth = input
                .trim()
                .parse()
                .expect ("breadth is not an integer");
    input.clear();

    print! ("Enter radius : ");
    io::stdout().flush().unwrap(); 
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    radius = input
                .trim()
                .parse()
                .expect ("radius is not an integer");
    input.clear();
    
    println! ("Rectangle : Area = {}, perimeter = {}", length * breadth, 2 * (length + breadth));
    println! ("Circle : circumference = {}, Area = {}", 2.0 * 3.14 * (radius as f32), 3.14 * (radius * radius) as f32);
}