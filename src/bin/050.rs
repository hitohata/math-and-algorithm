use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {

    input! {
        a: usize,
        b: usize
    }

    println!("{}", mod_pow(a, b))
}

fn mod_pow(a: usize, b: usize) -> usize {

    let mut answer = 1;
    let mut p = a;

    for i in 0..30 {
        if (b & 1 << i) != 0 {
            answer *= p;
            answer %= MOD;
        }
        p *= p;
        p %= MOD;
    }

    answer
}
