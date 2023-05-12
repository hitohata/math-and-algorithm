use proconio::input;

fn main() {
    input!{
        n: isize
    };

    let mut i = 2;
    let mut answer = true;

    while i * i <= n {
        if n % i == 0 {
            answer = false;
            break;
        }
        i += 1;
    }

    if answer {
        println!("{}", "Yes")
    } else {
        println!("{}", "No")
    }
}
