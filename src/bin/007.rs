use proconio::input;

fn main() {

    input! {
        n: isize,
        x: isize,
        y: isize
    }

    let mut sum = 0;

    for i in 1..=n {
        if i % x == 0 || i % y == 0 {
            sum += 1;
        };
    }

    println!("{}", sum);

}
