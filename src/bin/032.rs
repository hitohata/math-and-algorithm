use proconio::input;

fn main() {

    input! {
        n: usize,
        x: usize,
        _a: [usize; n]
    }

    let mut l = 0;
    let mut r = n;
    let mut is_exists = false;

    let mut a = _a.to_vec();
    a.insert(0, 0);

    a.sort();

    while l <= r {

        let m = (l + r) / 2;

        if a[m] == x {
            is_exists = true;
            break;
        }

        if a[m] > x {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    println!("{}", if is_exists {"Yes"} else {"No"})

}
