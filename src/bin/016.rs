use proconio::input;

fn main() {

    input! {
        n: usize,
        a: [usize; n]
    }

    let mut answer = a[0];

    for i in 1..n {

        answer = gcd(answer, a[i])

    }

    println!("{}", answer);


}

fn gcd(a: usize, b: usize) -> usize {

    if a > b { 
        let mod_val = a % b;

        if mod_val == 0 { return b };

        return gcd(mod_val, b);

    } else {
        let mod_val = b % a;

        if mod_val == 0 { return a };

        return gcd(a, mod_val);
    };

}
