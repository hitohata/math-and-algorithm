use proconio::input;

fn main() {

    input! {
        h: usize,
        w: usize,
    }


    if w == 1 || h == 1 {
        println!("{}", 1)
    } else {
        println!("{}", (h * w + 1) / 2)
    }

}
