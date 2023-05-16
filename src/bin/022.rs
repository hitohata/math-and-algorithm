use proconio::input;

fn main() {

    input! {
        n: usize,
        a: [usize; n]
    }

    let mut answer = 0;
    let mut count = vec![0; 100_001];

    for i in 0..n {
        count[a[i]] += 1;
    }

    if count[100_000 / 2] > 1 {
        answer += combination(count[100_000 / 2]);
        count[100_000 / 2] = 0;
    }


    for i in 0..100_000 / 2 {

        if count[i] * count[100_000 - i] > 0 {
            answer += count[i] * count[100_000 - i];
        }
    }

    println!("{}", answer);

}

fn combination(n: usize) -> usize {
    if n < 2 {
        return 0;
    }

    return n * (n - 1) / 2;
}
