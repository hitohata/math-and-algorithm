use std::vec;

use proconio::input;

fn main() {

    input! {
        h: usize,
        w: usize
    }

    let mut origin = vec![vec![0; w]; h];

    for r in 0..h {
        for c in 0..w {
            input! {
                a: usize
            }

            origin[r][c] = a;
        }
    }

    let mut row_sum = vec![0; h];
    let mut col_sum = vec![0; w];

    for r in 0..h {
        row_sum[r] = origin[r].iter().sum();
        for c in 0..w {
            col_sum[c] += origin[r][c];
        }
    }

    for r in 0..h {
        for c in 0..w {
            print!("{} ", row_sum[r] + col_sum[c] - origin[r][c]);
        }
        println!();
    }

}
