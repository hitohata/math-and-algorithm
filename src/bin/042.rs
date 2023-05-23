use proconio::input;

fn main() {

    input! {
        n: usize
    }

    let mut answer = 0;

    let mut f = vec![0_usize; n + 1];

    for i in 1..=n {
        for j in (i..=n).step_by(i) {
            f[j] += 1;
        }
    }

    for i in 1..=n {
        answer += i * f[i];
    }

    println!("{}", answer);
}
