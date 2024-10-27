/*
 * fill the entire screen with smiley face.
 * Consider screen size 100*100.
 * ASCII of smiley face is 1.
 *
 * In rust, 1 didn't print anything since 1 is not smiley face in standard ASCII, but in extended ASCII.
 * We used 
 */

fn main() {
    for _i in 1..=100 {
        for _j in 1..=100 {
            print! ("{}", '\u{263A}');
        }
        println!("");  // for new line
    }
}