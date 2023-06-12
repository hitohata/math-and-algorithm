use proconio::input;

fn main() {

    input! {
        n: usize,
        a_in: [usize; n]
    }

    let mut a = a_in.to_vec();
    a.insert(0, 0);

    let sum = (0..=n).map(|i| (a[i] as isize) * (2 * (i as isize ) - 1 - n as isize)).sum::<isize>();

    println!("{}", sum);

}
