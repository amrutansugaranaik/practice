/*
 * Give a table for this formula.
 * i = 2 + (y + 0.5 x)
 * y from 1 to 6
 * x from 5.5 to 12.5 with step 0.5
 */

const STEP:f32=0.5;
fn main () {

    for y in 1..=6 {
        let mut x:f32 = 5.5;
        while x <= 12.5 {
            let res:f32 = (0.5 * x) + (y as f32) + (2.0);
            println! ("{} {} {} ", y, x, res);
            x += STEP;
        }
    }
}