use proconio::input;

fn main() {

    input! {
        n: i64
    }

    let mut answer = Vec::<i64>::new();
    let mut i = 1;

    while i * i <= n {

        if n % i == 0 {
            answer.push(i);
            answer.push(n/i);
        }
        i += 1;
    };

    answer.sort();
    answer.dedup();

    for a in answer {
        println!("{}", a);
    }

}
