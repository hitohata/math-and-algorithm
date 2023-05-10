use proconio::input;

fn main() {

    input! {
        n: isize,
        _a: [isize; n]
    }

    let a = _a.to_vec();

    let mut sum = 0;

    for val in a.iter() {
        sum += val;
    }

    println!("{}", sum % 100);

}
