use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {

    input! {
        n: usize
    }

    let mut fib = [1, 1];

    for _ in 2..n {
        let sum = (fib[0] + fib[1]) % MOD;
        fib[0] = fib[1];
        fib[1] = sum;
    }

    println!("{}", fib[1]);

}
