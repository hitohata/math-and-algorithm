use proconio::input;

fn main() {

    input! {
        n: usize,
        a: [f64; n],
        b: [f64; n]
    }

    let a_possibility: f64 = 2.0 / 6.0;
    let b_possibility: f64 = 4.0 / 6.0;

    let answer = (0..n).map(|i| a_possibility * a[i] + b_possibility * b[i]).sum::<f64>();

    println!("{0:.12}", answer);

}
