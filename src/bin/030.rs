use std::cmp::max;
use proconio::input;

fn main() {

    input! {
        n: usize,
        w: usize,
        wn: [(usize, isize); n]
    }

    let mut weight = wn.iter().map(|x| x.0).collect::<Vec<usize>>();
    let mut value = wn.iter().map(|x| x.1).collect::<Vec<isize>>();

    weight.insert(0, 0);
    value.insert(0, 0);

    let mut dp = vec![vec![std::isize::MIN; w + 1]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        for j in 0..=w {

            if j < weight[i] {
                dp[i][j] = dp[i - 1][j];
            }

            if j >= weight[i] {
                dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - weight[i]] + value[i]);
            }
        }
    }

    let mut answer = 0;

    for i in 0..=n {
        for j in 0..=w {
            answer = max(answer, dp[i][j])
        }
    }

    println!("{}", answer);

}
