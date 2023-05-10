use proconio::input;

fn main() {
    input!{
        n: isize,
        s: isize,
        _a: [isize; n]
    };

    let a = _a.to_vec();

    let mut answer = false;

    for i in 0..1<<n {
        let mut sum = 0;
        for j in 0..n {

            if i & 1 << j != 0 {
                sum += a[j as usize]
            }

        }

        if sum == s {
            answer = true;
            break;
        }

    }

    if answer {
        println!("Yes")
    } else {
        println!("No")
    }

}
