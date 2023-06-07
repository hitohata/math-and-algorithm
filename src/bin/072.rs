use proconio::input;

fn main() {

    input! {
        a: usize,
        b: usize
    }

    let mut answer = 0;

    for i in 1..=b {
        if check(&a, &b, i) {
            answer = i;
        }
    }

    println!("{}", answer);

}

#[inline]
fn check(a: &usize, b: &usize, t: usize) -> bool {
    let left = ((a + t - 1) / t) as isize;
    let right = (b / t) as isize;

    if right - left >= 1 {
        true
    } else {
        false
    }
}
