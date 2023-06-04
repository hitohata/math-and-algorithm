use proconio::input;

fn main() {

    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let sum: usize = a.to_vec().iter().sum();

    if k < sum {
        println!("No");
        return;
    }

    if k == sum {
        println!("Yes");
        return;
    }

    if (k - sum) % 2 == 0 {
        println!("Yes")
    } else {
        println!("No")
    }

}
