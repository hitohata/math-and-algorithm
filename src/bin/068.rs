use proconio::input;

fn main() {

    input! {
        n: usize,
        k: usize,
        v: [usize; k]
    }

    let mut answer = 0;

    for i in 1..1<<k {

        let mut count = 0;
        let mut latest_common_multiple = 1;

        for j in 0..k {
            if (i & 1 << j) != 0 {
                count += 1;
                latest_common_multiple = lcm(latest_common_multiple, v[j]);
            }
        }

        let num = n / latest_common_multiple;

        if count % 2 == 1 {
            answer += num;
        } else {
            answer -= num;
        };

    }

    println!("{}", answer);

}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
}

fn lcm(a: usize, b: usize) -> usize {
    (a / gcd(a, b)) * b
}
