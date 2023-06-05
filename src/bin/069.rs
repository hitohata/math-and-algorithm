use std::cmp::max;
use proconio::input;

fn main() {

    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize
    }

    println!("{}", max(max(a * c, a * d), max(b * c, b * d)));

}
