use proconio::input;

fn main() {

    input! {
        n: usize,
        a: [usize; n]
    }

    let mut dp1 = vec![0_usize; n + 1];
    let mut dp2 = vec![0_usize; n + 1];

    for i in 1..=n {
        dp1[i] = dp2[i - 1] + a[i - 1];
        dp2[i] = std::cmp::max(dp1[i - 1], dp2[i - 1]);

    }

    println!("{}", std::cmp::max(dp1[n], dp2[n]));

}
