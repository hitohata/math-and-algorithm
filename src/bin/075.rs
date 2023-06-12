use std::vec;

use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {

    input! {
        n: usize,
        a_in: [usize; n]
    }

    let mut a = a_in.to_vec();
    a.insert(0, 0);

    let fact: Vec<usize> = fact(n);

    let mut sum = 0;

    for i in 1..=n {
        sum += a[i] * ncr(n - 1, i - 1, &fact);
        sum %= MOD;
    }

    println!("{}", sum);

}

fn ncr(n: usize, r: usize, fact: &Vec<usize>) -> usize {
    divider(fact[n], fact[r] * fact[n - r] % MOD) % MOD
}

fn fact(n: usize) -> Vec<usize> {

    let mut fact = vec![1; 1];

    for i in 1..n {
        fact.push(i * fact[i - 1] % MOD);
    }

    fact
}

fn divider(n: usize, r: usize) -> usize {
    n * mod_pow(r, MOD - 2) % MOD
}

fn mod_pow(a: usize, n: usize) -> usize {
    let mut p = a;
    let mut ans = 1;

    for i in 0..30 {
        if n & (1 << i) != 0 {
            ans *= p;
            ans %= MOD;
        }

        p *= p;
        p %= MOD;

    }

    ans
}
