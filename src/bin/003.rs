use proconio::input;

fn main() {
    input! {
        n: isize,
        _a: [isize; n]
    }

    let a: Vec<isize> = _a.to_vec();

    let mut sum = 0_isize;

    for val in a.iter() {
        sum += val;
    };

    println!("{}", sum);

}
