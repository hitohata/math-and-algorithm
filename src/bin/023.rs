use proconio::input;

fn main() {

    input! {
        n: usize,
        b: [usize; n],
        r: [usize; n]
    }

    let b_sum = b.into_iter().fold(0, |acc, x| acc + x) as f64;
    let r_sum = r.into_iter().fold(0, |acc, x| acc + x) as f64;

    println!("{}", b_sum / n as f64 + r_sum / n as f64)

}
