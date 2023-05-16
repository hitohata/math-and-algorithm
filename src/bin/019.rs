use proconio::input;

fn main() {

    input! {
        n: usize,
        a: [usize; n]
    }

    let mut answer = 0;
    let mut cards = [0usize; 4];
    a.into_iter().for_each(|el| cards[el] += 1);

    for i in 1..=3 {

        if cards[i] < 2 {
            continue;
        };

        answer += cards[i] * (cards[i] - 1) / 2;
    }

    println!("{}", answer);

}
