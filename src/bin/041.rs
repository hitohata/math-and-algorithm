use proconio::input;

fn main() {

    input! {
        t: usize,
        n: usize,
    }

    let mut w = vec![0_isize; t + 2];

    for _ in 0..n {

        input! {
            l: usize,
            r: usize
        }

        w[l] += 1;
        w[r] -= 1;

    }

    let mut s: isize = 0;

    for i in 0..t {
        if i == 0 {
            s = w[0];
        } else {
            s += w[i]
        }
        println!("{}", s);
    }
}
