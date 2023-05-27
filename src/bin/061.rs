use proconio::input;

fn main() {

    input! {
        n: usize
    }

    if n % 3 == 0 {
        println!("Second");
    } else {
        println!("First");
    }

}
