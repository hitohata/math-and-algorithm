use proconio::input;

fn main() {

    input! {
        n: usize,
        xy: [(isize, isize); n]
    }

    let mut smallest: f64 = std::f64::MAX;

    for i in 0..n - 1 {
        for j in i+1..n {
            smallest = if smallest < distance(xy[i], xy[j]) {
                smallest
            } else {
                distance(xy[i], xy[j])
            }
        }
    }

    println!("{0:.12}", smallest);

}

fn distance(a: (isize, isize), b: (isize, isize)) -> f64 {
    return f64::sqrt(((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as f64);
}
