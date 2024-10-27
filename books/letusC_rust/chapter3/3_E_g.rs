/*
 * Print this pattern.
 *       1
 *      2 3
 *     4 5 6
 *    7 8 9 10
 */

 fn main() {
    let mut space:i32 = 3;
    let mut tot_num_to_print:i32 = 1;
    let mut start_num:i32 = 1;

    while space >= 0 {
        for _i in 0..space {
            print! (" ");
        }
        space -= 1;

        for _i in 0..tot_num_to_print {
            print! ("{} ", start_num);
            start_num += 1;
        }
        println! ("");
        tot_num_to_print += 1;
    }
 }