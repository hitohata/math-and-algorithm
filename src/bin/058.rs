use proconio::input;

fn main() {
    input! {
        n: isize,
        x: isize,
        y: isize
    }

    if x.abs() + y.abs() <= n && (x + y) % 2 == n % 2 {
        println!("Yes");
    } else {
        println!("No")
    }

}
