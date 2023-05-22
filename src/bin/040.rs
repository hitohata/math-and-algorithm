use proconio::input;

fn main() {

    input! {
        n: usize,
        a: [isize; n - 1],
        m: usize
    }

    let mut dist = a.to_vec();
    dist.insert(0, 0);

    for i in 0..(n - 1) {
        dist[i + 1] += dist[i];
    };

    let mut answer = 0;

    let mut current_station = 0;

    for i in 0..m {
        input! {
            b: usize
        }

        if i == 0 {
            current_station = b;
            continue;
        }

        let d = isize::abs(dist[b - 1] - dist[current_station - 1]);

        answer += d;
        current_station = b;

    }

    println!("{}", answer);

}
