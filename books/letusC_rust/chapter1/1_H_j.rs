/*
 * A town has 80000 people. Men percentage is 52%, Women percentage is 48%.
 * 48% of total population in literate.
 * Literate men is 35% of total population. Find total number of illiterate men and women
 */

const TOTAL:i32 = 80000;

fn main () {
    let men:i32 = TOTAL * 52 / 100;
    //let women:i32 = TOTAL - men;
    let literate:i32 = TOTAL * 48 / 100;
    let illiterate:i32 = TOTAL - literate;
    let literate_men:i32 = TOTAL * 35 / 100;
    let illiterate_men = men - literate_men;
    let illiterate_women = illiterate - illiterate_men;

    println! ("illiterate men = {} ", illiterate_men);
    println! ("illiterate women = {} ", illiterate_women);

    // illiterate men = 13600 
    // illiterate women = 28000
    // total = 41600 --- Must be 52% of people
    // TOTAL * 52 / 100 = 41600
}