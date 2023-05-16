use proconio::input;

fn main() {

    input! {
        n: usize,
        a: [usize; n]
    }

    let mut ans = vec![0usize; 5];
    a.iter().for_each(|el| ans[el / 100] += 1);

    println!("{}", ans[1] * ans[4] + ans[2] * ans[3])

}
