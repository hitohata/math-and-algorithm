use proconio::input;

fn main() {
    input! {
        n: isize
    }

    let mut primes = (2..=n)
        .take_while(|&j| j * j <=n)
        .filter(|j| n % j == 0)
        .map(|j| vec![j, n / j])
        .flatten()
        .collect::<Vec<_>>();

    primes.sort();
    primes.dedup();

    let mut i = 0;
    let mut divided = n;
    let mut answer = Vec::<isize>::new();

    loop {
        if divided % primes[i] != 0 {
            i += 1;
            continue;
        }

        answer.push(primes[i]);

        divided = divided % primes[i];

        if primes.contains(&divided) {
            answer.push(divided);
            break;
        }
    }

    for i in answer {
        println!("{}", i);
    }

}
