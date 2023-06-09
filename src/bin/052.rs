use proconio::input;

const MOD: isize = 1_000_000_007;

fn main() {

    input! {
        x: isize,
        y: isize
    }

    if 2 * y - x < 0 || 2 * x - y < 0 {
        println!("0");
        return;
    }

    if (2 * y - x) % 3 != 0 || (2 * x - y) % 3 != 0 {
        println!("0");
        return;
    }

    let a = (2 * y - x) / 3;
    let b = (2 * x - y) / 3;

    let numerator = (1..=(a + b)).fold(1, |acc, x| acc * x % MOD);
    let denominator = (1..=a).fold((1..=b).fold(1, |acc, x| acc * x % MOD), |acc, x| acc * x % MOD);

    println!("{}", divider(numerator, denominator))

}

fn divider(numerator: isize, denominator: isize) -> isize {

    numerator * mod_pow(denominator, MOD - 2) % MOD

}

fn mod_pow(multiplicand: isize, multiplier: isize) -> isize {
    let mut a = 1;
    let mut p = multiplicand;

    for i in 0..30 {
        if (multiplier & 1 << i) != 0 {
            a *= p;
            a %= MOD;
        }
        p *= p;
        p %= MOD;
    }

    a
}
