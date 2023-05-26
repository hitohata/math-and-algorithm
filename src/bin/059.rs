use proconio::input;

fn main() {

    input! {
        n: usize
    }

    println!("{}", match n % 4 {
        1 => 2,
        2 => 4,
        3 => 8,
        _ => 6
    })

}
