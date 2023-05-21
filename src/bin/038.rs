use proconio::input;

fn main() {

    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut total = vec![0; n + 1];

    for i in 0..n {
        total[i + 1] = total[i] + a[i];
    }

    for _ in 0..q {

        input! {
            l: usize,
            r: usize
        }

        println!("{}", total[r] - total[l - 1]);

    }

}
