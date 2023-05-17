use proconio::input;

fn main() {

    input! {
        n: usize
    }

    let answer = (1..=n).map(|x| 1.0 / x as f64).sum::<f64>();

    println!("{0:.12}", answer * n as f64);

}
