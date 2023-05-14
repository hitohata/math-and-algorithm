use proconio::input;

fn main() {

    input! {
        n: usize,
        a: [usize; n]
    }

    let mut answer = a[0];

    for i in 1..n {
        answer = lcd(answer, a[i])
    }

    println!("{}", answer);

}

fn lcd(a: usize, b: usize) -> usize {

    return a / gcd(a, b) * b;

}

fn gcd(a: usize, b: usize) -> usize {

    if a > b {
        let mod_val = a % b;

        if mod_val == 0 {
            return b;
        }

        return gcd(a % b, b);
    } else {
        let mod_val = b % a;

        if mod_val == 0 {
            return a;
        }

        return gcd(a, mod_val)
    }
}
