use std::cmp::{min, max};

use proconio::input;

fn main() {

    input! {
        n: isize,
        k: isize
    }

    let mut sum = 0;

    for i in 1..=n {
        let a_min = i - (k - 1);
        let a_max = i + (k - 1);
        for b in max(1, a_min)..=min(n, a_max) {
            for c in max(1, a_min)..=min(n, a_max) {
                if isize::abs(b - c) <= k - 1 {
                    sum += 1;
                }
            }
        }
    }

    println!("{}", n.pow(3) - sum);

}
