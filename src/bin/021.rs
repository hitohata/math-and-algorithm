use proconio::input;

fn main() {

    input! {
        n: usize,
        r: usize
    }


    let mut numerator = 1;
    let mut denominator = 1;

    for i in 0..r {
        numerator *= n - i;
        denominator *= r - i;
    }

    println!("{}", numerator / denominator);

}
