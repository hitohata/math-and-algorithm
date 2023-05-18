use proconio::input;

fn main() {

    input! {
        n: usize
    }

    let mut dp = vec![0_usize; n + 1];

    for i in 0..=n {

        if i < 2 {
            dp[i] = 1;
        } else {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
    }

    println!("{}", dp[n]);

}
