use proconio::input;

fn main() {
    input!{
        n: isize
    };

    let mut answer = 1;

    for i in 1..=n {
        answer *= i
    }

    println!("{}", answer);

}
