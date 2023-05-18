use std::cmp::min;

use proconio::input;

fn main() {

    input! {
        n: usize,
        h_input: [isize; n]
    }

    let mut dp = vec![0_isize; n];
    let mut h = h_input.to_vec();

    dp.insert(0, 0);
    h.insert(0, 0);


    for i in 1..=n {

        match i {
            1 => dp[i] = 0,
            2 => dp[i] = (h[i - 1] - h[i]).abs(),
            _ => {

                let v1 = dp[i - 1] + (h[i - 1] - h[i]).abs();
                let v2 = dp[i - 2] + (h[i - 2] - h[i]).abs();

                dp[i] = min(v1, v2);
            }
        }

    }

    println!("{}", dp[n]);

}
