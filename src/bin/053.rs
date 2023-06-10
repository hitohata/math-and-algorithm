use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {

    input! {
        n: usize
    }


    let numerator = mod_pow(4,n + 1) - 1;
    let denominator = 3;

    println!("{}", divider(numerator, denominator));

}

fn divider(numerator: usize, denominator: usize) -> usize {
    numerator * mod_pow(denominator, MOD - 2) % MOD
}

fn mod_pow(a: usize, b: usize) -> usize {

    let mut ans = 1;
    let mut p = a;

    for i in 0..60 {

        if (b & 1 << i) != 0 {

            ans *= p;
            ans %= MOD;

        }

        p *= p;
        p %= MOD;
    }

    ans
}
