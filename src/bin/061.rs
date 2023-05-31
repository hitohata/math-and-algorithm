use proconio::input;

fn main() {

    input! {
        n: usize
    }

    let mut answer = false;

    for i in 1..=60 {
        if n == (1 << i) - 1 {
            answer = true;
        }
    }

    if answer {
        println!("Second");
    } else {
        println!("First");
    }

}
