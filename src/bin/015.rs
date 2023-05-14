use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize
    }

    if a > b {
        println!("{}", gcd(a, b))
    } else {
        println!("{}", gcd(b, a))
    }

}

fn gcd(numerator: isize, denominator: isize) -> isize {

    let mod_value = numerator % denominator;

    if mod_value == 0 {
        return denominator;
    }

    return gcd(denominator, mod_value);

}
