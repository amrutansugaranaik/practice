/*
 * Print this pattern.
 *
 * ABCDEFGFEDCBA
 * ABCDEF FEDCBA
 * ABCDE   EDCBA
 * ABCD     DCBA
 * ABC       CBA
 * AB         BA
 * A           A
 */

fn main() {
    let mut space:i32 = 1;
    let mut tot:i32 = 6;  // Total alphabets to print
    let mut last_char:char = 'G';

    // print first line as a hack. If two G were allowed, we could increase tot and last_char by 1.
    println! ("ABCDEFGFEDCBA");

    loop {

        // Print in sequence
        for i in 0..tot {
            print! ("{}", ('A' as u8 + i as u8) as char);
        }

        // Print spaces
        for _i in 1..=space {
            print! ("{}", " ");
        }

        // Print in sequence in reverse
        for i in 1..=tot {
            print! ("{}", (last_char as u8 - i as u8) as char);
        }

        println! ("");  // New line

        tot -= 1;
        space += 2;
        last_char = ((last_char as u8) - 1) as char;
        if tot == 0 {
            break;
        }
    }
}