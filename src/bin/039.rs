use proconio::input;

fn main() {

    input! {
        n: usize,
        q: usize
    }

    let mut snow = vec![0_isize; n + 1];

    for _ in 1..=q {
        input! {
            l: isize,
            r: isize,
            x: isize
        }

        for j in l -1..r {
            snow[j as usize] += x;
        }

    }

    for i in 1..n {
        let result = snow[i - 1] - snow[i];

        if result < 0 {
            print!("<");
        }
        else if result > 0 {
            print!(">");
        }
        else {
            print!("=")
        }
    }

}
