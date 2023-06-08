use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {

    input! {
        x: usize,
        y: usize
    }

    let numerator: usize =  (1..=(x + y)).fold(1, |acc, x| acc * x % MOD);
    let x_factional: usize = (1..=x).fold(1, |acc, x| acc * x % MOD);
    let y_factional: usize = (1..=y).fold(1, |acc, x| acc * x % MOD);

    println!("{}", divider(numerator, (x_factional * y_factional) % MOD))
}

fn divider(numerator: usize, denominator: usize) -> usize {
    numerator * mod_pow(denominator, MOD - 2) % MOD
}


fn mod_pow(a: usize, b: usize) -> usize {
    let mut ans = 1;
    let mut p = a;

    for i in 0..30 {
        if (b & 1 << i) != 0 {
            ans *= p;
            ans %= MOD;
        }
        p *= p;
        p %= MOD;
    }

    ans
}
