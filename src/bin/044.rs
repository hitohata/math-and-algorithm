use std::collections::VecDeque;
use proconio::input;

fn main() {

    input! {
        n: usize,
        m: usize
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for _ in 1..=m {

        input! {
            a: usize,
            b: usize
        }

        graph[a].push(b);
        graph[b].push(a);
    }

    let mut distance = vec![-1; n + 1];
    let mut queue: VecDeque<usize> = VecDeque::new();

    queue.push_back(1);
    distance[1] = 0;

    while !queue.is_empty() {

        let pos = queue.pop_front().unwrap();

        for i in 0..graph[pos].len() {

            let next = graph[pos][i];

            if distance[next] == -1 {
                distance[next] = distance[pos] + 1;
                queue.push_back(next);
            }

        };

    }

    for i in 1..=n {
        println!("{}", distance[i]);
    }

}
