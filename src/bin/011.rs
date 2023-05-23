use proconio::input;

fn main() {
    input!{
        n: usize
    };

    let mut primes = vec![true; n + 1];

    let mut i = 2;

    while i * i <= n {
        if primes[i] {
            for j in ((i * 2)..=n).step_by(i) {
                primes[j] = false;
            }
        }
        i += 1;
    }

    (2..=n).for_each(|i| {
        if primes[i] {
            print!("{} ", i)
        }
    });

}
