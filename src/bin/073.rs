use proconio::input;

const MOD: usize = 1000000007;

fn main() {

    input! {
        n: usize,
        a: [usize; n]
    }

    let mut answer = 0;
    let mut s = 1;

    for i in 0..n {
        answer += a[i] * s;
        answer %= MOD;
        s *= 2;
        s %= MOD;

    }

    println!("{}", answer);

}
