use proconio::input;

fn main() {

    input! {
        n: usize,
        q: usize
    }

    let mut snow = vec![0_isize; n + 2];

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            x: isize
        }

        snow[l] += x;
        snow[r + 1] -= x;

    }

    for i in 1..n {
        let result = snow[i + 1];

        if result < 0 {
            print!(">");
        }
        else if result > 0 {
            print!("<");
        }
        else {
            print!("=")
        }
    }

}
