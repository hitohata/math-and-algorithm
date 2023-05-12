use proconio::input;

fn main() {

    input! {
        n: i64
    }

    let mut answer = (1..=n)
        .take_while(|&j| j * j <= n)
        .filter(|j| n % j == 0)
        .map(|j| vec![j, n / j])
        .flatten()
        .collect::<Vec<_>>();

    answer.sort();
    answer.dedup();

    for an in answer {
        println!("{}", an);
    }
}
